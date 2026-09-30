"use client";

import { RiAddLine, RiCheckLine, RiSparklingLine } from "@remixicon/react";
import { useEffect, useState } from "react";

import {
  Dropdown,
  DropdownGroup,
  DropdownItem,
  DropdownPopover,
  DropdownTrigger,
} from "@/components/base/dropdown/dropdown";
import { ModelInstallModal } from "@/components/application/agent-chat/model-install-modal";
import { cx } from "@/utils/cx";

export interface OllamaModel {
  id: string;
  label: string;
  size: string;
}

/**
 * Picks which locally pulled model answers.
 *
 * The list comes from /api/models, which reads Ollama's own tags endpoint - so
 * it is whatever is actually on this machine, never a hardcoded menu that can
 * drift from reality. With Ollama down the trigger says so instead of offering
 * models that cannot run.
 */
export function ModelSwitcher({
  value,
  onChange,
  className,
}: {
  value: string | null;
  onChange: (id: string) => void;
  className?: string;
}) {
  const [models, setModels] = useState<OllamaModel[]>([]);
  const [state, setState] = useState<"loading" | "ready" | "empty" | "unavailable">("loading");
  const [isOpen, setIsOpen] = useState(false);
  const [installOpen, setInstallOpen] = useState(false);

  useEffect(() => {
    let cancelled = false;
    fetch("/api/models")
      .then((res) => (res.ok ? res.json() : null))
      .then((data: { models?: OllamaModel[] } | null) => {
        if (cancelled) return;
        const list = data?.models ?? [];
        setModels(list);
        // "empty" is Ollama answering with nothing pulled - a different problem
        // from Ollama not answering at all, and the only one the user can fix
        // from here.
        setState(list.length > 0 ? "ready" : data ? "empty" : "unavailable");
        // Nothing chosen yet: adopt whatever Ollama listed first, so the
        // trigger never reads "Select model" while a model is answering.
        if (list.length > 0 && !value) onChange(list[0].id);
      })
      .catch(() => {
        if (!cancelled) setState("unavailable");
      });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps -- fetch the list once on mount
  }, []);

  const label =
    state === "loading"
      ? "Loading models"
      : state === "empty"
        ? "Add a model"
        : state === "unavailable"
          ? "Ollama offline"
          : (value ?? "Select model");

  if (state !== "ready") {
    // Nothing pulled: the row becomes the way to fix that. Offline or still
    // loading stays inert - there is nothing useful to click.
    const actionable = state === "empty";
    return (
      <>
        <button
          type="button"
          disabled={!actionable}
          onClick={() => setInstallOpen(true)}
          title={state === "unavailable" ? "Start Ollama, then reload" : undefined}
          className={cx(
            "flex items-center gap-1.5 rounded-lg px-1.5 py-0.5 text-body-2-medium text-text-tertiary",
            actionable
              ? "cursor-pointer hover:bg-background-secondary-hover hover:text-text-secondary"
              : "cursor-default",
            className,
          )}
        >
          <RiSparklingLine className="size-4 shrink-0" aria-hidden />
          {label}
          {actionable && <RiAddLine className="size-4 shrink-0" aria-hidden />}
        </button>
        {installOpen && (
          <ModelInstallModal hasModels={false} onClose={() => setInstallOpen(false)} />
        )}
      </>
    );
  }

  return (
    <>
    <Dropdown isOpen={isOpen} onOpenChange={setIsOpen}>
      <DropdownTrigger
        aria-label={`Model: ${label}`}
        className={cx(
          "flex cursor-pointer items-center gap-1.5 rounded-lg px-1.5 py-0.5 text-body-2-medium text-text-secondary",
          "transition-colors hover:bg-background-secondary-hover",
          className,
        )}
      >
        <RiSparklingLine className="size-4 shrink-0 text-foreground-icon-secondary" aria-hidden />
        <span className="max-w-[18ch] truncate">{label}</span>
      </DropdownTrigger>

      <DropdownPopover aria-label="Choose a model" placement="top start">
        <DropdownGroup>
          <DropdownItem
            onSelect={() => {
              setIsOpen(false);
              setInstallOpen(true);
            }}
          >
            <span className="flex w-full items-center gap-2">
              <RiAddLine className="size-4 shrink-0 text-foreground-icon-secondary" aria-hidden />
              <span className="flex-1 truncate text-text-secondary">Add a model</span>
            </span>
          </DropdownItem>
          {models.map((model) => (
            <DropdownItem
              key={model.id}
              selected={model.id === value}
              onSelect={() => {
                onChange(model.id);
                setIsOpen(false);
              }}
            >
              <span className="flex w-full items-center gap-2">
                <span className="min-w-0 flex-1 truncate">{model.label}</span>
                <span className="shrink-0 text-body-2-regular text-text-tertiary">
                  {model.size}
                </span>
                {model.id === value && (
                  <RiCheckLine className="size-4 shrink-0 text-foreground-icon-secondary" aria-hidden />
                )}
              </span>
            </DropdownItem>
          ))}
        </DropdownGroup>
      </DropdownPopover>
      </Dropdown>
      {installOpen && (
        <ModelInstallModal hasModels onClose={() => setInstallOpen(false)} />
      )}
    </>
  );
}
