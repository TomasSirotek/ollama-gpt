"use client";

import { RiArrowDownSLine, RiSparkling2Line, RiStopFill } from "@remixicon/react";
import { useEffect, useState } from "react";

import { AgentThinking } from "@/components/application/agent-thinking/agent-thinking";
import { cx } from "@/utils/cx";

/**
 * What the model is doing before it answers.
 *
 * Reasoning models (qwen3, deepseek-r1) stream a separate reasoning channel
 * ahead of the reply - the route forwards it with `sendReasoning`. Shown as a
 * live elapsed timer that expands into the reasoning itself, so a long pause
 * reads as work in progress rather than a stall.
 *
 * Once the run finishes the indicator goes STATIC. AgentThinking's dots animate
 * on an infinite CSS loop, so leaving it mounted after the answer landed made a
 * finished reply look like it was still working - and kept a compositor
 * animation running for no reason.
 *
 * Deliberately not a task checklist: a checklist implies discrete steps, and
 * discrete steps only exist when the model is calling tools. Inventing them
 * from a prose stream would be decoration pretending to be telemetry.
 */
export function AgentReasoning({
  text,
  streaming,
  onStop,
  className,
}: {
  /** The reasoning stream so far. Empty until the model emits any. */
  text: string;
  /** Still running: drives the timer, the shimmer, and the stop button. */
  streaming: boolean;
  /** Aborts the run in flight. */
  onStop?: () => void;
  className?: string;
}) {
  const [expanded, setExpanded] = useState(false);
  const seconds = useElapsedSeconds(streaming);

  if (!streaming && !text) return null;

  const elapsed = seconds > 0 ? `${seconds.toFixed(1)}s` : null;

  return (
    <div className={cx("flex flex-col gap-2", className)}>
      <div className="flex w-fit items-center gap-1">
        <button
          type="button"
          onClick={() => setExpanded((open) => !open)}
          disabled={!text}
          aria-expanded={expanded}
          className={cx(
            "flex items-center gap-1.5 rounded-lg px-1 py-0.5 text-left transition-colors",
            text ? "cursor-pointer hover:bg-background-secondary-hover" : "cursor-default",
          )}
        >
          {streaming ? (
            <AgentThinking variant="wave" label={elapsed ? `Thinking ${elapsed}` : "Thinking"} />
          ) : (
            // Static twin of the running state: same row, no animation.
            <span className="flex items-center gap-1.5 text-body-medium text-text-tertiary">
              <RiSparkling2Line className="size-4 shrink-0" aria-hidden />
              Thought process
              {elapsed && <span className="text-body-2-regular">{elapsed}</span>}
            </span>
          )}
          {text && (
            <RiArrowDownSLine
              className={cx(
                "size-4 shrink-0 text-foreground-icon-tertiary transition-transform",
                expanded && "rotate-180",
              )}
              aria-hidden
            />
          )}
        </button>

        {streaming && onStop && (
          <button
            type="button"
            onClick={onStop}
            aria-label="Stop generating"
            title="Stop generating"
            className="flex size-6 shrink-0 cursor-pointer items-center justify-center rounded-full bg-background-secondary-default text-foreground-icon-secondary transition-colors hover:bg-background-secondary-hover"
          >
            <RiStopFill className="size-3.5" aria-hidden />
          </button>
        )}
      </div>

      {expanded && text && (
        <div className="max-h-60 overflow-y-auto rounded-xl bg-background-secondary-default px-3 py-2.5">
          <p className="text-body-2-regular whitespace-pre-wrap text-text-secondary">{text}</p>
        </div>
      )}
    </div>
  );
}

/**
 * Elapsed time since the run started, ticking while it runs and holding its
 * final value after, so a finished row still reports how long it took.
 */
function useElapsedSeconds(running: boolean): number {
  const [seconds, setSeconds] = useState(0);

  useEffect(() => {
    if (!running) return;
    const startedAt = Date.now();
    // The first tick 100ms later replaces any value held from the previous
    // run, so there is nothing to reset synchronously here.
    // 100ms: fast enough to read as live, slow enough to cost nothing while
    // the GPU is busy with the model.
    const id = setInterval(() => setSeconds((Date.now() - startedAt) / 1000), 100);
    return () => clearInterval(id);
  }, [running]);

  return seconds;
}
