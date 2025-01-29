// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { Button as ButtonPrimitive } from "@kobalte/core/button";
import * as DialogPrimitive from "@kobalte/core/dialog";
import type { PolymorphicProps } from "@kobalte/core/polymorphic";
import type { ComponentProps, JSX, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";

import { cn } from "../../lib/utils";

const Root = DialogPrimitive.Root;

type DialogCloseButtonXProps<T extends ValidComponent = "button"> =
  DialogPrimitive.DialogCloseButtonProps<T> & { class?: string | undefined };

function CloseButtonX<T extends ValidComponent = "button">(
  props: PolymorphicProps<T, DialogCloseButtonXProps<T>>,
) {
  const [, rest] = splitProps(props as DialogCloseButtonXProps, ["class"]);
  return (
    <ButtonPrimitive
      class={cn(
        "absolute top-margin right-margin rounded-sm text-neutral-11 hover:text-neutral-12 disabled:pointer-events-none",
        props.class,
      )}
      {...rest}
    >
      <svg
        class="size-4"
        fill="none"
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path d="M18 6l-12 12" />
        <path d="M6 6l12 12" />
      </svg>
      <span class="sr-only">Close</span>
    </ButtonPrimitive>
  );
}

function Portal(props: DialogPrimitive.DialogPortalProps) {
  const [, rest] = splitProps(props, ["children"]);
  return (
    <DialogPrimitive.Portal {...rest}>
      <div class="fixed inset-0 z-50 flex items-start justify-center sm:items-center">
        {props.children}
      </div>
    </DialogPrimitive.Portal>
  );
}

type DialogOverlayProps<T extends ValidComponent = "div"> =
  DialogPrimitive.DialogOverlayProps<T> & { class?: string | undefined };

function Overlay<T extends ValidComponent = "div">(
  props: PolymorphicProps<T, DialogOverlayProps<T>>,
) {
  const [, rest] = splitProps(props as DialogOverlayProps, ["class"]);
  return (
    <DialogPrimitive.Overlay
      class={cn("fixed inset-0 z-50 bg-overlay-10", props.class)}
      {...rest}
    />
  );
}

type DialogContentProps<T extends ValidComponent = "div"> =
  DialogPrimitive.DialogContentProps<T> & {
    class?: string | undefined;
    children?: JSX.Element;
  };

function Content<T extends ValidComponent = "div">(
  props: PolymorphicProps<T, DialogContentProps<T>>,
) {
  const [, rest] = splitProps(props as DialogContentProps, ["class"]);
  return (
    <Portal>
      <Overlay />
      <DialogPrimitive.Content
        class={cn(
          "-translate-x-1/2 -translate-y-3/4 fixed top-1/2 left-1/2 z-50 grid max-h-screen w-full max-w-2xl gap-4 overflow-y-auto border-2 border-neutral-7 bg-neutral-3 p-gutter text-neutral-12 shadow-lg sm:rounded-lg",
          props.class,
        )}
        {...rest}
      />
    </Portal>
  );
}

function Header(props: ComponentProps<"div">) {
  const [, rest] = splitProps(props, ["class"]);
  return (
    <div
      class={cn(
        "flex flex-col space-y-1.5 text-center sm:text-left",
        props.class,
      )}
      {...rest}
    />
  );
}

function Footer(props: ComponentProps<"div">) {
  const [, rest] = splitProps(props, ["class"]);
  return (
    <div class={cn("flex flex-row justify-between", props.class)} {...rest} />
  );
}

type DialogTitleProps<T extends ValidComponent = "h2"> =
  DialogPrimitive.DialogTitleProps<T> & {
    class?: string | undefined;
  };

function Title<T extends ValidComponent = "h2">(
  props: PolymorphicProps<T, DialogTitleProps<T>>,
) {
  const [, rest] = splitProps(props as DialogTitleProps, ["class"]);
  return (
    <DialogPrimitive.Title
      class={cn(
        "font-semibold text-lg leading-none tracking-tight",
        props.class,
      )}
      {...rest}
    />
  );
}

type DialogDescriptionProps<T extends ValidComponent = "p"> =
  DialogPrimitive.DialogDescriptionProps<T> & {
    class?: string | undefined;
  };

function Description<T extends ValidComponent = "p">(
  props: PolymorphicProps<T, DialogDescriptionProps<T>>,
) {
  const [, rest] = splitProps(props as DialogDescriptionProps, ["class"]);
  return (
    <DialogPrimitive.Description
      class={cn("text-neutral-11 text-sm", props.class)}
      {...rest}
    />
  );
}

export const Dialog = Object.assign(Root, {
  CloseButtonX,
  Content,
  Header,
  Footer,
  Title,
  Description,
});
