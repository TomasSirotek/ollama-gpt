"use client";

import {
  RiAddLine,
  RiDownloadLine,
  RiSearchLine,
  RiSideBarFill,
} from "@remixicon/react";
import { useState } from "react";

import {
  AccountMenu,
  ThreadRow,
  type ChatThreadSummary,
} from "@/components/application/agent-chat/agent-chat-history";
import { SettingsModal } from "@/components/application/settings/settings-modal";
import { CloseButton } from "@/components/base/buttons/close-button";
import { Kbd } from "@/components/base/kbd/kbd";
import { cx } from "@/utils/cx";

/**
 * The app's only rail: brand, search, new chat, then the chat history.
 *
 * Replaces the starter's split of a nav sidebar on the left and a history rail
 * on the right - with one destination there is nothing to navigate between, so
 * the history is the navigation and belongs in the same column as everything
 * else that acts on it.
 */
export function ChatSidebar({
  threads,
  activeId,
  onSelect,
  onNewChat,
  onRename,
  onToggleUnread,
  onDelete,
  onExport,
  onOpenSearch,
  disabled = false,
  mobile = false,
  onClose,
  className,
}: {
  threads: ChatThreadSummary[];
  activeId: string;
  onSelect: (id: string) => void;
  onNewChat: () => void;
  onRename: (id: string, title: string) => void;
  onToggleUnread: (id: string) => void;
  onDelete: (id: string) => void;
  onExport: () => void;
  onOpenSearch: () => void;
  /** Disables switching mid-stream, which would strand the running response. */
  disabled?: boolean;
  /** Rendered inside the phone drawer: close button instead of nothing. */
  mobile?: boolean;
  onClose?: () => void;
  className?: string;
}) {
  const exportLabel = threads.length === 0 ? "No chats to export" : `Export ${threads.length} chats`;
  // The drawer is already a temporary overlay, so collapsing inside it would
  // only shrink something the visitor is about to dismiss.
  const [collapsedState, setCollapsed] = useState(false);
  const collapsed = mobile ? false : collapsedState;
  const [settingsOpen, setSettingsOpen] = useState(false);

  return (
    <aside
      aria-label="Chats"
      className={cx(
        "flex h-full shrink-0 flex-col gap-3 overflow-hidden rounded-3xl bg-background-secondary-default p-3",
        "transition-[width] duration-300 ease-in-out",
        collapsed ? "w-15" : "w-65",
        className,
      )}
    >
      <div className="flex items-center justify-between gap-2 px-1 pt-1">
        {!collapsed && (
          <span className="truncate text-body-large text-title-3-bold text-text-primary">
            OllamaGPT
          </span>
        )}
        {mobile && onClose ? (
          <CloseButton onClick={onClose} aria-label="Close navigation" />
        ) : (
          <button
            type="button"
            onClick={() => setCollapsed((open) => !open)}
            aria-label={collapsed ? "Show sidebar" : "Hide sidebar"}
            title={collapsed ? "Show sidebar" : "Hide sidebar"}
            aria-expanded={!collapsed}
            className="flex size-6 shrink-0 cursor-pointer items-center justify-center rounded-lg text-foreground-icon-secondary transition-colors hover:bg-background-secondary-hover"
          >
            <RiSideBarFill className="size-5" aria-hidden />
          </button>
        )}
      </div>

      <button
        type="button"
        onClick={onOpenSearch}
        aria-label="Search chats"
        title={collapsed ? "Search chats" : undefined}
        className="flex w-full cursor-pointer items-center gap-2 rounded-2lg bg-background-tertiary-default px-2 py-2 text-left transition-colors hover:bg-background-secondary-hover"
      >
        <RiSearchLine className="size-5 shrink-0 text-foreground-icon-secondary" aria-hidden />
        {!collapsed && (
          <>
            <span className="min-w-0 flex-1 truncate text-body-medium text-text-secondary">
              Search chats
            </span>
            <Kbd>⌘K</Kbd>
          </>
        )}
      </button>

      <button
        type="button"
        onClick={onNewChat}
        disabled={disabled}
        title={collapsed ? "New chat" : undefined}
        className={cx(
          "flex w-full cursor-pointer items-center gap-2 rounded-2lg px-2 py-2 text-body-medium text-white",
          "bg-linear-to-b from-accent-500 to-accent-600 shadow-nav-selected",
          "transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50",
        )}
      >
        <RiAddLine className="size-5 shrink-0" aria-hidden />
        {!collapsed && "New chat"}
      </button>

      <div
        className={cx(
          "flex min-h-0 flex-1 flex-col gap-1 overflow-y-auto scrollbar-none",
          collapsed && "invisible",
        )}
      >
        <p className="px-2 pt-2 pb-1 text-body-2-medium text-text-tertiary">Recent</p>

        {threads.length === 0 ? (
          <p className="px-2 text-body-2-regular text-text-tertiary">
            Chats you start show up here.
          </p>
        ) : (
          threads.map((thread) => (
            <ThreadRow
              key={thread.id}
              thread={thread}
              active={thread.id === activeId}
              disabled={disabled}
              onSelect={onSelect}
              onRename={onRename}
              onToggleUnread={onToggleUnread}
              onDelete={onDelete}
            />
          ))
        )}
      </div>

      <div className="mt-auto flex flex-col gap-2 border-t border-separator-border pt-3">
        <div className={cx("flex items-center gap-1 pr-1", collapsed && "justify-center")}>
          <AccountMenu threads={threads} onOpenSettings={() => setSettingsOpen(true)} />
          <button
            type="button"
            onClick={onExport}
            disabled={threads.length === 0}
            hidden={collapsed}
            aria-label={exportLabel}
            title={exportLabel}
            className="flex size-6 shrink-0 cursor-pointer items-center justify-center rounded-full bg-button-primary text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-40"
          >
            <RiDownloadLine className="size-3.5 shrink-0" aria-hidden />
          </button>
        </div>
      </div>

      <SettingsModal isOpen={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </aside>
  );
}
