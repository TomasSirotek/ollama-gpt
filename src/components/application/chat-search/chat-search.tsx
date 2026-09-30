"use client";

import { RiChatAiLine, RiSearchLine } from "@remixicon/react";
import { useEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import type { ChatThreadSummary } from "@/components/application/agent-chat/agent-chat-history";
import { cx } from "@/utils/cx";

/**
 * Command-palette search over the stored chats.
 *
 * Opened with cmd/ctrl-K from anywhere, or the sidebar's search row. A single
 * field centred over a blurred backdrop, results underneath, arrow keys and
 * Enter to pick - the shape people already know from every editor.
 *
 * Titles are all we have to match on: thread bodies live in localStorage per
 * thread and loading every one of them to search would cost more than it is
 * worth at this size.
 */
export function ChatSearch({
  onClose,
  threads,
  onSelect,
}: {
  onClose: () => void;
  threads: ChatThreadSummary[];
  onSelect: (id: string) => void;
}) {
  const [query, setQuery] = useState("");
  const [cursor, setCursor] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  const results = useMemo(() => {
    const q = query.trim().toLocaleLowerCase();
    if (!q) return threads;
    return threads.filter((t) => t.title.toLocaleLowerCase().includes(q));
  }, [query, threads]);

  // The parent mounts this only while the palette is open, so state starts
  // fresh every time - no reset effect, and nothing to get out of sync.
  // The portal content lands in document.body after this component commits, so
  // focus is claimed on the next frame rather than synchronously on mount.
  useEffect(() => {
    const id = requestAnimationFrame(() => {
      const input = inputRef.current;
      if (input) input.focus({ preventScroll: true });
    });
    return () => cancelAnimationFrame(id);
  }, []);

  // Keep the highlighted row in view as the cursor moves past the fold.
  useEffect(() => {
    listRef.current
      ?.querySelector('[data-active="true"]')
      ?.scrollIntoView({ block: "nearest" });
  }, [cursor]);

  if (typeof document === "undefined") return null;

  const choose = (id: string) => {
    onSelect(id);
    onClose();
  };

  const onKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === "Escape") {
      event.preventDefault();
      onClose();
      return;
    }
    if (event.key === "ArrowDown") {
      event.preventDefault();
      setCursor((c) => (results.length === 0 ? 0 : (c + 1) % results.length));
      return;
    }
    if (event.key === "ArrowUp") {
      event.preventDefault();
      setCursor((c) => (results.length === 0 ? 0 : (c - 1 + results.length) % results.length));
      return;
    }
    if (event.key === "Enter") {
      event.preventDefault();
      const hit = results[cursor];
      if (hit) choose(hit.id);
    }
  };

  return createPortal(
    <div
      className="fixed inset-0 z-100 flex items-start justify-center px-4 pt-[18vh]"
      role="presentation"
      onKeyDown={onKeyDown}
    >
      {/* Blurred backdrop - click anywhere outside to dismiss. */}
      <button
        type="button"
        aria-label="Close search"
        tabIndex={-1}
        onClick={onClose}
        className="absolute inset-0 cursor-default bg-black/40 backdrop-blur-md"
      />

      <div
        role="dialog"
        aria-modal="true"
        aria-label="Search chats"
        className="relative flex w-full max-w-[560px] flex-col overflow-hidden rounded-3xl bg-background-secondary-default shadow-dropdown ring-1 ring-separator-border"
      >
        <div className="flex items-center gap-2.5 px-4 py-3.5">
          <RiSearchLine className="size-5 shrink-0 text-foreground-icon-secondary" aria-hidden />
          <input
            ref={inputRef}
            value={query}
            onChange={(event) => {
              setQuery(event.target.value);
              setCursor(0);
            }}
            placeholder="Search chats"
            aria-label="Search chats"
            className="min-w-0 flex-1 bg-transparent text-body-medium text-text-primary outline-none placeholder:text-text-tertiary"
          />
          <kbd className="shrink-0 rounded-md bg-background-tertiary-default px-1.5 py-0.5 text-body-2-medium text-text-tertiary">
            esc
          </kbd>
        </div>

        {results.length > 0 && (
          <div
            ref={listRef}
            className="flex max-h-[45vh] flex-col gap-0.5 overflow-y-auto border-t border-separator-border p-2"
          >
            {results.map((thread, index) => (
              <button
                key={thread.id}
                type="button"
                data-active={index === cursor}
                onMouseEnter={() => setCursor(index)}
                onClick={() => choose(thread.id)}
                className={cx(
                  "flex w-full cursor-pointer items-center gap-2.5 rounded-2lg px-2 py-2 text-left transition-colors",
                  index === cursor ? "bg-background-secondary-hover" : "bg-transparent",
                )}
              >
                <RiChatAiLine
                  className="size-4 shrink-0 text-foreground-icon-secondary"
                  aria-hidden
                />
                <span className="min-w-0 flex-1 truncate text-body-medium text-text-primary">
                  {thread.title}
                </span>
                <span className="shrink-0 text-body-2-regular text-text-tertiary">
                  {relativeTime(thread.updatedAt)}
                </span>
              </button>
            ))}
          </div>
        )}

        {results.length === 0 && (
          <p className="border-t border-separator-border px-4 py-6 text-center text-body-2-regular text-text-tertiary">
            {threads.length === 0 ? "No chats yet." : "No chats match that."}
          </p>
        )}
      </div>
    </div>,
    document.body,
  );
}

/** Opens the palette on cmd/ctrl-K anywhere in the app. */
export function useChatSearchShortcut(onOpen: () => void) {
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key.toLowerCase() === "k" && (event.metaKey || event.ctrlKey)) {
        event.preventDefault();
        onOpen();
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [onOpen]);
}

function relativeTime(at: number): string {
  const mins = Math.max(0, Math.round((Date.now() - at) / 60000));
  if (mins < 1) return "now";
  if (mins < 60) return `${mins}m`;
  const hours = Math.round(mins / 60);
  if (hours < 24) return `${hours}h`;
  return `${Math.round(hours / 24)}d`;
}
