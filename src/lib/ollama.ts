import { createOpenAICompatible } from '@ai-sdk/openai-compatible';

/**
 * Ollama serves an OpenAI-compatible API at /v1, so the official provider works
 * against it directly - no Ollama-specific package needed.
 *
 * Note the /v1 suffix: /api is Ollama's own protocol, /v1 is the OpenAI one.
 */
export const ollama = createOpenAICompatible({
  name: 'ollama',
  baseURL: process.env.OLLAMA_BASE_URL ?? 'http://localhost:11434/v1',
  apiKey: 'ollama', // required by the OpenAI shape, ignored by a local server
  // Ollama supports response_format: json_schema, but the provider assumes not
  // and silently drops the schema unless told otherwise. Without this,
  // generateObject sends no schema at all and the model free-writes prose.
  supportsStructuredOutputs: true,
});

export const DEFAULT_MODEL = process.env.OLLAMA_DEFAULT_MODEL ?? 'qwen3:4b';
