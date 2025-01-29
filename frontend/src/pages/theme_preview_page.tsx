// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { createSignal, For } from "solid-js";

import { Button } from "../components/ui/button";
import { Dialog } from "../components/ui/dialog";
import { Table } from "../components/ui/table";
import { TextField } from "../components/ui/text-field";

export function ThemePreviewPage() {
  const [dialog_open, set_dialog_open] = createSignal(false);

  const table_example_projects = [
    {
      name: "mortal combat v2",
      format: "cube",
      date_modified: "2024-12-13",
    },
    {
      name: "fundamentos",
      format: "cube",
      date_modified: "2025-04-20",
    },
    {
      name: "amulet titan",
      format: "modern",
      date_modified: "2025-05-08",
    },
    {
      name: "br darcy",
      format: "modern",
      date_modified: "2023-05-02",
    },
    {
      name: "optimized b1 vega",
      format: "commander",
      date_modified: "2025-03-18",
    },
  ];

  return (
    <main class="mx-auto flex w-min flex-col items-center gap-margin p-margin">
      <h1 class="font-bold text-2xl">Button</h1>
      <div class="flex flex-col items-center rounded bg-neutral-2 p-4">
        <h2 class="mb-2 font-semibold">Small</h2>
        <div class="flex justify-center gap-gutter">
          <Button size="sm" variant="neutral">
            Neutral
          </Button>
          <Button size="sm" variant="accent">
            Accent
          </Button>
          <Button size="sm" variant="success">
            Success
          </Button>
          <Button size="sm" variant="warning">
            Warning
          </Button>
          <Button size="sm" variant="danger">
            Danger
          </Button>
        </div>
        <h2 class="mb-2 font-semibold">Default</h2>
        <div class="flex justify-center gap-gutter">
          <Button variant="neutral">Neutral</Button>
          <Button variant="accent">Accent</Button>
          <Button variant="success">Success</Button>
          <Button variant="warning">Warning</Button>
          <Button variant="danger">Danger</Button>
        </div>
        <h2 class="mb-2 font-semibold">Large</h2>
        <div class="flex justify-center gap-gutter">
          <Button size="lg" variant="neutral">
            Neutral
          </Button>
          <Button size="lg" variant="accent">
            Accent
          </Button>
          <Button size="lg" variant="success">
            Success
          </Button>
          <Button size="lg" variant="warning">
            Warning
          </Button>
          <Button size="lg" variant="danger">
            Danger
          </Button>
        </div>
      </div>
      <h1 class="font-bold text-2xl">Dialog</h1>
      <div class="flex h-md w-lg flex-col items-center justify-center rounded bg-neutral-2">
        <Dialog onOpenChange={set_dialog_open} open={dialog_open()}>
          <Button onMouseDown={() => set_dialog_open(true)}>Trigger</Button>
          <Dialog.Content>
            <Dialog.CloseButtonX onMouseDown={() => set_dialog_open(false)} />
            <Dialog.Header>
              <Dialog.Title>Title</Dialog.Title>
              <Dialog.Description>
                Ad Lorem consequat elit incididunt cillum.
              </Dialog.Description>
            </Dialog.Header>
            <div class="grid gap-gutter py-gutter">
              <TextField class="grid grid-cols-4 items-center gap-gutter">
                <TextField.Label class="text-right">
                  Project name
                </TextField.Label>
                <TextField.Input
                  class="col-span-3"
                  type="text"
                  value="Fundamentos"
                />
              </TextField>
              <TextField class="grid grid-cols-4 items-center gap-gutter">
                <TextField.Label class="text-right">
                  Description
                </TextField.Label>
                <TextField.Input
                  class="col-span-3"
                  type="text"
                  value="this is a description"
                />
              </TextField>
            </div>
            <Dialog.Footer>
              <Button
                onMouseDown={() => set_dialog_open(false)}
                variant="danger"
              >
                Cancel
              </Button>
              <Button
                onMouseDown={() => set_dialog_open(false)}
                variant="success"
              >
                Submit
              </Button>
            </Dialog.Footer>
          </Dialog.Content>
        </Dialog>
      </div>
      <h1 class="font-bold text-2xl">Table</h1>
      <Table>
        <Table.Header>
          <Table.Row>
            <Table.Head>Name</Table.Head>
            <Table.Head>Format</Table.Head>
            <Table.Head>Date Modified</Table.Head>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          <For each={table_example_projects}>
            {(project) => (
              <Table.Row>
                <Table.Cell class="font-medium">{project.name}</Table.Cell>
                <Table.Cell>{project.format}</Table.Cell>
                <Table.Cell>{project.date_modified}</Table.Cell>
              </Table.Row>
            )}
          </For>
        </Table.Body>
      </Table>
    </main>
  );
}
