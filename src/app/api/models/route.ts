/** Ollama's own host, without whichever API path the env var carries.
 *  OLLAMA_BASE_URL points at the OpenAI-compatible surface (/v1), but the
 *  model list only exists on Ollama's native /api/tags. */
function ollamaHost(): string {
  const raw = process.env.OLLAMA_BASE_URL ?? 'http://localhost:11434';
  try {
    return new URL(raw).origin;
  } catch {
    return 'http://localhost:11434';
  }
}

export async function GET() {
  try {
    const res = await fetch(`${ollamaHost()}/api/tags`, { cache: 'no-store' });
    if (!res.ok) throw new Error(`Ollama replied ${res.status}`);

    const data: { models?: { name: string; size: number }[] } = await res.json();
    const models = (data.models ?? []).map((m) => ({
      id: m.name,
      label: m.name,
      size: `${(m.size / 1e9).toFixed(1)}GB`,
    }));

    return Response.json({ models });
  } catch {
    // The switcher renders "Ollama offline" rather than offering models that
    // cannot answer.
    return Response.json({ models: [], error: 'Ollama is not available' });
  }
}
