// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import type { ComponentProps } from "solid-js";
import { splitProps } from "solid-js";

import { cn } from "../../lib/utils";

function Root(props: ComponentProps<"table">) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <div class="relative w-full overflow-auto">
      <table
        class={cn("w-full caption-bottom text-sm", local.class)}
        {...others}
      />
    </div>
  );
}

function Header(props: ComponentProps<"thead">) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <thead class={cn("[&_tr]:hover:bg-inherit", local.class)} {...others} />
  );
}

function Body(props: ComponentProps<"tbody">) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <tbody class={cn("[&_tr:last-child]:border-0", local.class)} {...others} />
  );
}

function Footer(props: ComponentProps<"tfoot">) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <tfoot
      class={cn("bg-neutral-3 font-medium text-neutral-12", local.class)}
      {...others}
    />
  );
}

function Row(props: ComponentProps<"tr">) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <tr
      class={cn(
        "border-neutral-6 border-b hover:bg-neutral-2 data-[state=selected]:bg-neutral-3",
        local.class,
      )}
      {...others}
    />
  );
}

function Head(props: ComponentProps<"th">) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <th
      class={cn(
        "h-10 px-2 text-left align-middle font-medium text-neutral-11 [&:has([role=checkbox])]:pr-0",
        local.class,
      )}
      {...others}
    />
  );
}

function Cell(props: ComponentProps<"td">) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <td
      class={cn("p-2 align-middle [&:has([role=checkbox])]:pr-0", local.class)}
      {...others}
    />
  );
}

function Caption(props: ComponentProps<"caption">) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <caption
      class={cn("mt-4 text-neutral-11 text-sm", local.class)}
      {...others}
    />
  );
}

export const Table = Object.assign(Root, {
  Header,
  Body,
  Footer,
  Head,
  Row,
  Cell,
  Caption,
});
