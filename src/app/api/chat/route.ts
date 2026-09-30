import {
  convertToModelMessages,
  createUIMessageStreamResponse,
  streamText,
  toUIMessageStream,
  type UIMessage,
} from 'ai';
import { ollama, DEFAULT_MODEL } from '@/lib/ollama';

export const maxDuration = 60;

export async function POST(req: Request) {
  try {
    const { messages, model }: { messages: UIMessage[]; model?: string } = await req.json();

    const result = streamText({
      model: ollama(model ?? DEFAULT_MODEL),
      system: 'You are a helpful, concise assistant. Answer questions clearly and accurately.',
      // useChat sends UIMessages (with parts); the model wants ModelMessages.
      messages: await convertToModelMessages(messages),
    });

    // The standalone helpers are the current API: the result.toUIMessageStream*
    // methods are deprecated and go away in the next major.
    return createUIMessageStreamResponse({
      stream: toUIMessageStream({
        stream: result.stream,
        sendReasoning: true,
        // errors are masked to "An error occurred." by default
        onError: (error) => {
          console.error('[chat]', error);
          return error instanceof Error ? error.message : String(error);
        },
      }),
    });
  } catch (error) {
    if (error instanceof Error && error.message.includes('ECONNREFUSED')) {
      return new Response(
        JSON.stringify({
          error: 'Ollama is not running. Start it with: ollama serve',
        }),
        { status: 503, headers: { 'Content-Type': 'application/json' } }
      );
    }
 
    return new Response(
      JSON.stringify({ error: 'An unexpected error occurred' }),
      { status: 500, headers: { 'Content-Type': 'application/json' } }
    );
  }
}