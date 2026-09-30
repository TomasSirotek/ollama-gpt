# Billing simulation — step by step

A plan to build usage metering, plans, limits and Stripe checkout into this app.
Build it in order. Every step works on its own and is verifiable before the next.

---

## 0. What this is, and the honest caveat

**We are simulating how an LLM product charges**, using a local model that costs
nothing to run.

Real LLM pricing exists because every token costs the vendor money. Your GPU is
already paid for, so metering it is theatre — commercially. Three reasons to build
it anyway:

- It is the cheapest possible way to learn the mechanics, with no API bill while
  you get it wrong.
- A real version of this product would not be local. Hosted means an API bill or
  rented GPUs, and metering becomes real.
- Even self-hosted you meter — for fair-share between tenants and abuse limits,
  not for cost.

**Pitch it as per-seat with fair-use limits**, not per-token billing. That is what
self-hosted products actually charge, and it is more credible than a token meter.

---

## 1. Usage metering (no Stripe at all)

**What:** count tokens per user per billing period, and store them.

**Why first:** this is the whole product minus payments. Stripe is mostly
configuration once this works, and if metering is wrong the billing is wrong.

### 1.1 Database

SQLite + Drizzle is plenty. No hosted DB for a demo.

```
users    id, email, plan, periodStart, periodEnd, stripeCustomerId
usage    userId, periodStart, tokensIn, tokensOut, requests
```

`periodStart` on usage is what makes a reset possible: you never delete usage, you
start counting against a new period.

### 1.2 Record on finish, not on send

In `src/app/api/chat/route.ts`, `streamText` takes an `onFinish` callback that
receives the real token counts from the provider:

```ts
const result = streamText({
  model: ollama(model ?? DEFAULT_MODEL),
  messages: await convertToModelMessages(messages),
  onFinish: async ({ usage }) => {
    await recordUsage(userId, usage.inputTokens, usage.outputTokens);
  },
});
```

**Two things matter here.**

You cannot know the cost before generating, so recording on send would be a guess.

`onFinish` fires even when the client disconnects mid-stream, so an aborted reply
still bills. That is correct — the tokens were generated.

### 1.3 Verify

Send three messages, then read the usage row. The numbers should be non-zero and
grow. Compare against what Ollama reports if you want to check they are real.

---

## 2. Plans and quota

**What:** define tiers, block requests over quota with a `402`.

### 2.1 Plans as data, not code

```ts
export const PLANS = {
  free: { tokens:    50_000, rpm:  5, models: ["llama3.2:1b", "qwen3:4b"] },
  pro:  { tokens: 1_000_000, rpm: 30, models: "all" },
  team: { tokens: 5_000_000, rpm: 60, models: "all" },
} as const;
```

Three tiers is the standard shape: an anchor, the one you want them on, and a
ceiling that makes the middle look reasonable.

### 2.2 The gate, in the route

Before `streamText`:

```
1. identify the user
2. load plan + usage for the current period
3. over quota           -> 402 with a message naming the plan
4. model not in tier    -> 403
5. otherwise, stream
```

Order matters: reject before doing expensive work, and return a *specific* error
so the UI can say "you are out of tokens" rather than "something went wrong".

### 2.3 Surfacing it

The composer status row already has a slot — put "42k of 50k tokens" there. On a
`402`, the UI should offer the upgrade rather than just failing.

### 2.4 Verify

Set the free quota to something tiny (1000 tokens). Chat until it trips. You
should get a `402` and a message that names the limit.

---

## 3. Rate limiting

**What:** cap requests per minute per user.

Quota stops someone using a month of tokens; rate limiting stops them using it in
ten seconds, and stops one user starving everyone else on a single GPU.

A sliding window in memory is fine for a demo:

```
requests per user, timestamps in the last 60s
over the plan's rpm -> 429, with a Retry-After header
```

**Verify:** fire a loop of requests. The `n+1`th should be a `429`.

---

## 4. Stripe setup (test mode only)

**Everything below happens in test mode.** No real card, no real money.

### 4.1 In the dashboard

1. Create a **Product** per paid plan (Pro, Team).
2. Give each a recurring **Price** (monthly).
3. Copy the `price_...` ids into `.env.local`.

### 4.2 Keys

```
STRIPE_SECRET_KEY=sk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...
STRIPE_PRICE_PRO=price_...
STRIPE_PRICE_TEAM=price_...
```

The `sk_test_` prefix is your safety rail — test keys cannot touch real money.

### 4.3 Local webhooks

```bash
stripe login
stripe listen --forward-to localhost:3000/api/webhooks/stripe
```

That prints the `whsec_...` for your env. Leave it running while you develop.

---

## 5. Checkout

**What:** `POST /api/billing/checkout` creates a Stripe Checkout Session and
returns its URL. The browser redirects there.

```ts
const session = await stripe.checkout.sessions.create({
  mode: "subscription",
  customer: user.stripeCustomerId,
  line_items: [{ price: PRICE_IDS[plan], quantity: 1 }],
  success_url: `${origin}/?checkout=success`,
  cancel_url: `${origin}/?checkout=cancelled`,
});
```

**Never build your own card form.** Card details must not touch your server — that
is the entire reason Checkout exists, and it is what keeps you out of PCI scope.

**Verify:** click upgrade, land on Stripe's page, pay with `4242 4242 4242 4242`,
get redirected back. Nothing has changed in your database yet — that is correct,
and step 6 is why.

---

## 6. Webhook — the only place access is granted

**What:** `POST /api/webhooks/stripe`, the single source of truth for entitlements.

Handle:

| event | meaning |
|---|---|
| `checkout.session.completed` | they paid the first time |
| `customer.subscription.updated` | plan changed, or renewed |
| `customer.subscription.deleted` | cancelled — drop to free |
| `invoice.paid` | period renewed — reset usage |

### 6.1 Three things that bite everyone

**Verify the signature.** Anyone can POST to your endpoint. Without this, they can
grant themselves a Team plan with `curl`.

```ts
const event = stripe.webhooks.constructEvent(rawBody, signature, WEBHOOK_SECRET);
```

Next gives you a parsed body by default; `constructEvent` needs the **raw** text or
the signature will never match. Use `await req.text()`.

**Webhooks arrive at least once.** Stripe retries on any non-2xx, and sometimes
delivers twice anyway. Store every `event.id` you have processed and ignore repeats,
or a retry grants a second month of credits.

**Never grant access from the redirect.** A user landing on `/?checkout=success`
proves nothing — they can type that URL. The redirect is only a "thanks, this will
update shortly"; the webhook does the work.

### 6.2 Verify

```bash
stripe trigger checkout.session.completed
```

The user's plan should change in your database without a browser involved. Then
send the same event twice and confirm the second is ignored.

---

## 7. Billing UI

The Settings modal already has a **Billing** row waiting.

Show: current plan, usage this period with a bar, renewal date, and an upgrade or
manage button.

**Do not build cancel, payment-method, or invoice screens.** One API call gives you
all of it, hosted and maintained by Stripe:

```ts
const portal = await stripe.billingPortal.sessions.create({
  customer: user.stripeCustomerId,
  return_url: origin,
});
```

**Verify:** upgrade, see the plan change in Settings, open the portal, cancel, and
watch the webhook drop you back to free.

---

## 8. Credit top-ups (optional)

One-off payments that add tokens outside the plan quota, for the month someone
needs more without upgrading.

Same shape as step 5 with `mode: "payment"`, plus a `credits` table. Spend plan
quota first, then credits — otherwise someone burns credits they paid for while
their included allowance sits unused.

---

## 9. Test cards

| number | behaviour |
|---|---|
| `4242 4242 4242 4242` | succeeds |
| `4000 0000 0000 9995` | declined, insufficient funds |
| `4000 0025 0000 3155` | requires 3D Secure |
| `4000 0000 0000 0341` | attaches, then fails on charge |

Any future expiry, any CVC, any postcode.

---

## 10. Build order, condensed

1. metering — prove usage rows grow
2. plans + quota — prove the 402 fires
3. rate limiting — prove the 429 fires
4. Stripe products, keys, `stripe listen`
5. checkout — reach Stripe and come back
6. webhook — plan changes without a browser
7. billing UI in Settings
8. credits

Stages 1–3 are the interesting part and need no Stripe account. Do not start 4
until the 402 works.