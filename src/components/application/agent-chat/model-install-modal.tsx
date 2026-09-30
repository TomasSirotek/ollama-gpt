"use client";

import { RiCheckLine, RiCloseLine, RiFileCopyLine } from "@remixicon/react";
import { useEffect, useState } from "react";
import { createPortal } from "react-dom";

import { cx } from "@/utils/cx";

/**
 * How to get a model onto this machine.
 *
 * Opened when Ollama has nothing pulled, and from the "+" beside the model
 * picker at any time. Every row is a real `ollama pull` you can paste - the
 * sizes and descriptions are the only editorial content, and the command is
 * what actually does the work.
 *
 * There is no in-app install button on purpose: pulling a model is a
 * multi-gigabyte download on the user's own machine, and a web page kicking
 * that off from a click it cannot show progress for or cancel is worse than
 * handing over the command.
 */

export interface SuggestedModel {
  id: string;
  size: string;
  bestFor: string;
}

/** Sized for a 6-8GB card; anything larger needs its own judgement call. */
const SUGGESTED: SuggestedModel[] = [
  { id: "llama3.2:1b", size: "700MB", bestFor: "Fast responses, low-resource machines" },
  { id: "llama3.2", size: "2GB", bestFor: "General chat, good balance" },
  { id: "qwen3:4b", size: "2.5GB", bestFor: "Chain-of-thought reasoning" },
  { id: "mistral", size: "4GB", bestFor: "Strong reasoning, multilingual" },
  { id: "qwen2.5-coder:7b", size: "4.5GB", bestFor: "Code generation and review" },
];

export function ModelInstallModal({
  onClose,
  hasModels,
}: {
  onClose: () => void;
  /** Ollama answered with at least one model: this is "add another", not "get started". */
  hasModels: boolean;
}) {
  const [custom, setCustom] = useState("");

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [onClose]);

  if (typeof document === "undefined") return null;

  return createPortal(
    <div className="fixed inset-0 z-100 flex items-center justify-center p-4" role="presentation">
      <button
        type="button"
        aria-label="Close"
        tabIndex={-1}
        onClick={onClose}
        className="absolute inset-0 cursor-default bg-black/50 backdrop-blur-sm"
      />

      <div
        role="dialog"
        aria-modal="true"
        aria-label="Add a model"
        className="relative flex max-h-[min(620px,calc(100dvh-32px))] w-full max-w-[620px] flex-col overflow-hidden rounded-3xl bg-background-primary-default shadow-dropdown ring-1 ring-separator-border"
      >
        <div className="flex shrink-0 items-start justify-between gap-4 px-6 pt-6 pb-4">
          <div className="flex flex-col gap-1">
            <h2 className="text-title-3-medium text-text-primary">
              {hasModels ? "Add another model" : "No models installed"}
            </h2>
            <p className="text-body-2-regular text-text-secondary">
              {hasModels
                ? "Pull another model and it appears in the picker."
                : "Ollama is reachable but has nothing pulled yet. Run one of these."}
            </p>
          </div>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close"
            className="flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-lg text-foreground-icon-secondary transition-colors hover:bg-background-secondary-hover"
          >
            <RiCloseLine className="size-4" aria-hidden />
          </button>
        </div>

        <div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 pb-6">
          {/* Manual pull first: it is the base state, and the one command that
              works for any model on ollama.com, not just the five below. */}
          <section className="flex flex-col gap-2">
            <h3 className="text-body-medium text-text-primary">Pull any model</h3>
            <input
              value={custom}
              onChange={(event) => setCustom(event.target.value)}
              placeholder="model name, e.g. gemma3:4b"
              aria-label="Model name"
              className="w-full rounded-xl bg-background-secondary-default px-3 py-2 text-body-regular text-text-primary outline-none ring-1 ring-separator-border placeholder:text-text-tertiary focus:ring-2 focus:ring-border-focus-ring"
            />
            <Command value={`ollama pull ${custom.trim() || "<model>"}`} />
            <p className="text-body-2-regular text-text-tertiary">
              Browse everything at ollama.com/library. Restart nothing - the picker
              re-reads the list when you reopen it.
            </p>
          </section>

          <section className="flex flex-col gap-2">
            <h3 className="text-body-medium text-text-primary">Suggested</h3>
            <div className="overflow-hidden rounded-xl ring-1 ring-separator-border">
              {SUGGESTED.map((model, index) => (
                <div
                  key={model.id}
                  className={cx(
                    "flex items-center gap-3 px-3 py-2.5",
                    index > 0 && "border-t border-separator-border",
                  )}
                >
                  <div className="flex min-w-0 flex-1 flex-col">
                    <code className="w-fit rounded-md bg-background-secondary-default px-1.5 py-0.5 font-mono text-body-2-medium text-text-primary">
                      {model.id}
                    </code>
                    <span className="truncate text-body-2-regular text-text-tertiary">
                      {model.bestFor}
                    </span>
                  </div>
                  <span className="shrink-0 text-body-2-regular text-text-tertiary">
                    {model.size}
                  </span>
                  <CopyButton value={`ollama pull ${model.id}`} label={`Copy pull command for ${model.id}`} />
                </div>
              ))}
            </div>
          </section>

          {!hasModels && (
            <section className="flex flex-col gap-2">
              <h3 className="text-body-medium text-text-primary">Ollama not installed?</h3>
              <Command value="curl -fsSL https://ollama.com/install.sh | sh" />
              <p className="text-body-2-regular text-text-tertiary">
                On Arch: <code className="font-mono">sudo pacman -S ollama</code> then{" "}
                <code className="font-mono">sudo systemctl enable --now ollama</code>.
              </p>
            </section>
          )}
        </div>
      </div>
    </div>,
    document.body,
  );
}

function Command({ value }: { value: string }) {
  return (
    <div className="flex items-center gap-2 rounded-xl bg-background-secondary-default px-3 py-2">
      <code className="min-w-0 flex-1 overflow-x-auto font-mono text-body-2-regular whitespace-nowrap text-text-primary">
        {value}
      </code>
      <CopyButton value={value} label={`Copy: ${value}`} />
    </div>
  );
}

function CopyButton({ value, label }: { value: string; label: string }) {
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (!copied) return;
    const timer = setTimeout(() => setCopied(false), 1400);
    return () => clearTimeout(timer);
  }, [copied]);

  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      onClick={async () => {
        try {
          await navigator.clipboard.writeText(value);
          setCopied(true);
        } catch {
          // Clipboard blocked (insecure origin, denied permission): the command
          // is selectable text, so there is still a way to take it.
        }
      }}
      className="flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-lg text-foreground-icon-secondary transition-colors hover:bg-background-secondary-hover"
    >
      {copied ? (
        <RiCheckLine className="size-4 text-foreground-icon-secondary" aria-hidden />
      ) : (
        <RiFileCopyLine className="size-4" aria-hidden />
      )}
    </button>
  );
}
