// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { Button } from "../components/ui/button";

export function TestingPage() {
  return (
    <main class="mx-auto flex w-[80%] flex-col pt-10">
      <section>
        <h2 class="text-4xl leading-tight">Database</h2>
        <Button>re parse default_cards and load into temp database</Button>
      </section>
    </main>
  );
}
