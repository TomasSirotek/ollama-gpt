"use client";

import { memo, type ComponentPropsWithoutRef } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

import { cx } from "@/utils/cx";

/**
 * Renders a model reply as markdown instead of showing its source.
 *
 * Models emit markdown whether or not you ask for it - `**bold**`, `###`,
 * bullet lists, fenced code. Printed raw it reads as noise, which is what the
 * asterisks everywhere were.
 *
 * Every element maps to the design system's own tokens rather than a typography
 * plugin, so replies inherit the theme (including the amber accent on links)
 * and stay readable in both modes. Raw HTML in the model's output is NOT
 * enabled: react-markdown escapes it by default and that stays that way, since
 * the text comes from a model and is not trusted markup.
 */
export const Markdown = memo(function Markdown({ children }: { children: string }) {
  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        p: ({ children }) => (
          <p className="text-body-regular break-words text-text-primary">{children}</p>
        ),
        strong: ({ children }) => (
          <strong className="font-semibold text-text-primary">{children}</strong>
        ),
        em: ({ children }) => <em className="italic">{children}</em>,
        a: ({ children, href }) => (
          <a
            href={href}
            target="_blank"
            rel="noopener noreferrer"
            className="text-accent-600 underline underline-offset-2 hover:text-accent-700"
          >
            {children}
          </a>
        ),
        ul: ({ children }) => (
          <ul className="flex list-disc flex-col gap-1 pl-5 text-body-regular text-text-primary">
            {children}
          </ul>
        ),
        ol: ({ children }) => (
          <ol className="flex list-decimal flex-col gap-1 pl-5 text-body-regular text-text-primary">
            {children}
          </ol>
        ),
        li: ({ children }) => <li className="break-words">{children}</li>,
        h1: ({ children }) => (
          <h1 className="text-title-3-medium text-text-primary">{children}</h1>
        ),
        h2: ({ children }) => (
          <h2 className="text-title-3-medium text-text-primary">{children}</h2>
        ),
        h3: ({ children }) => (
          <h3 className="text-body-medium text-text-primary">{children}</h3>
        ),
        blockquote: ({ children }) => (
          <blockquote className="border-l-2 border-separator-border pl-3 text-body-regular text-text-secondary">
            {children}
          </blockquote>
        ),
        hr: () => <hr className="border-separator-border" />,
        // Fenced blocks arrive as <pre><code>; react-markdown 10 dropped the
        // `inline` prop, so the wrapper is what distinguishes the two.
        pre: ({ children }) => (
          <pre className="overflow-x-auto rounded-xl bg-background-tertiary-default p-3 text-body-2-regular text-text-primary">
            {children}
          </pre>
        ),
        code: ({ children, className }: ComponentPropsWithoutRef<"code">) => {
          const fenced = /language-/.test(className ?? "");
          return (
            <code
              className={cx(
                "font-mono",
                fenced
                  ? "bg-transparent p-0 text-body-2-regular"
                  : "rounded-md bg-background-tertiary-default px-1 py-0.5 text-body-2-regular",
              )}
            >
              {children}
            </code>
          );
        },
        table: ({ children }) => (
          <div className="overflow-x-auto">
            <table className="w-full border-collapse text-left text-body-2-regular">
              {children}
            </table>
          </div>
        ),
        th: ({ children }) => (
          <th className="border-b border-separator-border px-2 py-1.5 text-text-tertiary">
            {children}
          </th>
        ),
        td: ({ children }) => (
          <td className="border-b border-separator-border px-2 py-1.5 text-text-primary">
            {children}
          </td>
        ),
      }}
    >
      {children}
    </ReactMarkdown>
  );
});
