import { generateText, Output } from 'ai';
import { z } from 'zod';

import { DEFAULT_MODEL, ollama } from '@/lib/ollama';

const SentimentSchema = z.object({
  sentiment: z.enum(['positive', 'negative', 'neutral']),
  confidence: z.number().min(0).max(1),
  summary: z.string().max(200),
});

export async function POST(req: Request) {
  const { text } = await req.json();

  try {
    // Output.object is what enforces the schema: it goes to the model as a JSON
    // schema AND the reply is parsed through zod, so this either matches
    // SentimentSchema or throws. (generateObject does the same but is deprecated.)
    const { output } = await generateText({
      model: ollama(DEFAULT_MODEL),
      output: Output.object({ schema: SentimentSchema }),
      prompt: `Analyze the sentiment of this text: "${text}"`,
    });

    return Response.json(output);
  } catch (error) {
    // Thrown when the model returns something the schema rejects, even after
    // the SDK's retries. A small local model will do this sometimes.
    console.error('[analyze]', error);
    return Response.json(
      { error: 'The model did not return a valid result' },
      { status: 502 },
    );
  }
}
