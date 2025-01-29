// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { A } from "@solidjs/router";

export function NotFound() {
  return (
    <main class="flex h-screen flex-col items-center justify-center">
      <h1 class="mb-4 text-4xl">404 - Page Not Found</h1>
      <p class="mb-8 text-gray-dim">
        The page you're looking for doesn't exist.
      </p>
      <A
        class="cursor-pointer rounded bg-grass-4 px-4 py-2 hover:bg-grass-5 dark:bg-grassdark-4 dark:hover:bg-grassdark-5"
        href="/"
      >
        Return Home
      </A>
    </main>
  );
}
