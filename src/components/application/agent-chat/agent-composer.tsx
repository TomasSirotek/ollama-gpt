"use client";

import {
  RiArrowUpLine,
  RiAttachment2,
  RiInfinityLine,
  RiSparklingLine,
  RiStopFill,
} from "@remixicon/react";
import { useRef, type KeyboardEvent, type SyntheticEvent } from "react";

import { ComposerLoader } from "@/components/application/composer-loader/composer-loader";
import { ModelSwitcher } from "@/components/application/agent-chat/model-switcher";
import { cx } from "@/utils/cx";

/**
 * The free composer: a 52px pill with a text field and circular controls,
 * plus the thin status row beneath it.
 *
 * It matches the proportions of the Pro composer on purpose, so a starter
 * looks like it belongs to the same product — the fixed-height pill, 8px
 * padding with 36px circles sitting flush inside it, and the 10px gap rhythm
 * across the row. What it deliberately does NOT carry is the paid work in
 * `composer`: the liquid-glass surfaces behind the controls, the model and
 * effort menus, the attachment plugin panel, and the voice equalizer. Those
 * are the reason someone buys that component, so this one leaves the slots
 * present but plain.
 *
 * The status row shows what is actually true of the running app — which
 * provider answered, which model, how many messages are in the thread —
 * rather than decorative chrome, so it stays honest in a deployed starter.
 */

export interface AgentComposerProps {
  value: string;
  onValueChange: (value: string) => void;
  onSubmit: () => void;
  onStop: () => void;
  busy: boolean;
  /** Model id reported by the runtime, e.g. "openai/gpt-5-nano". */
  model?: string | null;
  /** Given, the status row renders a live model picker instead of a static badge. */
  onModelChange?: (id: string) => void;
  provider?: string | null;
  messageCount: number;
  className?: string;
}

export function AgentComposer({
  value,
  onValueChange,
  onSubmit,
  onStop,
  busy,
  model,
  onModelChange,
  provider,
  messageCount,
  className,
}: AgentComposerProps) {
  const inputRef = useRef<HTMLInputElement>(null);

  const submit = (event: SyntheticEvent) => {
    event.preventDefault();
    onSubmit();
  };

  const onKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      onSubmit();
    }
  };

  return (
    <div className={cx("flex w-full flex-col gap-2.5", className)}>
      {/* radius omitted: ComposerLoader defaults to a full pill, matching the form. */}
      {/* bloom=0 drops the widest layer: a 32px stroke blurred at radius 14
          across the whole pill, re-rasterised every frame. This loader only
          runs while the model is generating, so that filter competes with the
          GPU exactly when it has nothing spare. The tight glow and crisp line
          keep the effect; delete this prop to get the halo back. */}
      <ComposerLoader active={busy} bloom={0}>
        {/* The pill sits one step lighter than the chat surface behind it in
            both themes (white on neutral-100 in light, neutral-800 on
            neutral-900 in dark), so a single token works without a dark
            override. Its controls then step back down to secondary. */}
        <form
          onSubmit={submit}
          className={cx(
            "flex h-[52px] w-full items-center gap-2.5 rounded-full p-2",
            // ComposerLoader paints its own pill surface and runs the light
            // behind its children, so an opaque form would cover the light
            // entirely. While busy the form steps aside and lets it show.
            busy ? "bg-transparent" : "bg-background-primary-default shadow-xs",
          )}
        >
          {/* Attachment slot. Plain on purpose — the plugin panel behind this
              button in the Pro composer is part of what that component sells. */}
          {/* Shares the Pro composer's add-button tokens so the two read as the
              same control. They resolve one step lighter than the pill in dark
              and one step darker in light, which is why a single pair works for
              both themes without a dark: override. */}
          <button
            type="button"
            aria-label="Add attachment"
            className="flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-full bg-ai-chat-composer-add-background text-foreground-icon-primary transition-colors duration-150 ease hover:bg-ai-chat-composer-add-hover-background"
          >
            <RiAttachment2 className="size-5" aria-hidden />
          </button>

          <label className="sr-only" htmlFor="agent-composer-input">
            Message
          </label>
          <input
            id="agent-composer-input"
            ref={inputRef}
            type="text"
            value={value}
            onChange={(event) => onValueChange(event.target.value)}
            onKeyDown={onKeyDown}
            placeholder="Ask me anything"
            autoComplete="off"
            className="h-5 min-w-0 flex-1 bg-transparent text-body-regular text-text-primary caret-text-primary outline-none placeholder:text-text-tertiary"
          />

          <div className="flex shrink-0 items-center gap-2 pl-1.5">
            {busy ? (
              <button
                type="button"
                onClick={onStop}
                aria-label="Stop generating"
                className="flex size-9 cursor-pointer items-center justify-center rounded-full bg-background-secondary-default text-foreground-icon-secondary transition-colors hover:bg-background-secondary-hover"
              >
                <RiStopFill className="size-5" aria-hidden />
              </button>
            ) : (
              <button
                type="submit"
                aria-label="Send message"
                disabled={value.trim().length === 0}
                className="flex size-9 cursor-pointer items-center justify-center rounded-full bg-button-primary text-white transition-opacity disabled:cursor-not-allowed disabled:opacity-40"
              >
                <RiArrowUpLine className="size-5" aria-hidden />
              </button>
            )}
          </div>
        </form>
      </ComposerLoader>

      <div className="flex h-[26px] w-full items-center justify-between">
        <div className="flex items-center gap-3">
          {onModelChange ? (
            <ModelSwitcher value={model ?? null} onChange={onModelChange} />
          ) : (
            <StatusItem icon={RiInfinityLine} label={provider ?? "Not configured"} />
          )}
        </div>
        <div className="flex items-center gap-3">
          <StatusItem
            icon={RiSparklingLine}
            label={messageCount === 0 ? "New chat" : `${messageCount} messages`}
          />
        </div>
      </div>
    </div>
  );
}

function StatusItem({
  icon: Icon,
  label,
}: {
  icon: typeof RiInfinityLine;
  label: string;
}) {
  return (
    <span className="flex items-center gap-1">
      <Icon className="size-4 shrink-0 text-foreground-icon-secondary" aria-hidden />
      <span className="whitespace-nowrap text-body-2-medium text-text-secondary">{label}</span>
    </span>
  );
}

/** "openai/gpt-5-nano" reads better in a 13ch slot as "gpt-5-nano". */
