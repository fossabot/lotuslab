// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { action, createAsync, json, query, useAction } from "@solidjs/router";
import {
  type ColumnDef,
  createSolidTable,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
} from "@tanstack/solid-table";
import { createSignal, For, Show } from "solid-js";

import { commands, type Folder, type NewFolder } from "../bindings";
import { IconPlus } from "../components/icons";
import { Button } from "../components/ui/button";
import { Dialog } from "../components/ui/dialog";
import { Table } from "../components/ui/table";
import { TextField } from "../components/ui/text-field";

export function HomePage() {
  const rootChildren = createAsync(() => getRootFolderChildren());

  return (
    <main class="mx-auto flex w-[80%] flex-col pt-10">
      {/* Header */}
      <div class="mb-margin flex items-baseline justify-between">
        <h1 class="text-3xl leading-tight">Projects</h1>
        <NewFolderDialog />
      </div>

      {/* Projects */}
      <Show when={rootChildren()}>
        {(children) => <ProjectTable data={children().folders} />}
      </Show>
    </main>
  );
}

const getRootFolderChildren = query(
  () =>
    commands.getFolderChildren("folder:root").then((r) => {
      if (r.status === "ok") {
        return r.data;
      } else {
        console.error(r.error);
      }
    }),
  "get_folder_children",
);

const actionNewFolder = action((f: NewFolder) =>
  commands.newFolder(f).then((r) => {
    if (r.status === "ok") {
      return json(r.data, { revalidate: "get_folder_children" });
    } else {
      console.error(r.error);
    }
  }),
);

function NewFolderDialog() {
  const [dialogOpen, setDialogOpen] = createSignal(false);
  const [newFolderName, setNewFolderName] = createSignal("");

  const newFolder = useAction(actionNewFolder);

  async function handleNewFolder() {
    const created = await newFolder({ name: newFolderName(), parent: null });

    setDialogOpen(false);
    setNewFolderName("");

    if (!created) {
      console.error("failed to create folder");
    } else {
      console.log("successfully created new folder");
    }
  }

  return (
    <Dialog onOpenChange={setDialogOpen} open={dialogOpen()}>
      <Button
        onMouseDown={() => setDialogOpen(true)}
        size="icon"
        variant="success"
      >
        <IconPlus />
      </Button>
      <Dialog.Content
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            e.preventDefault();
            handleNewFolder();
          }
        }}
      >
        <Dialog.CloseButtonX onMouseDown={() => setDialogOpen(false)} />
        <Dialog.Header>
          <Dialog.Title>New Folder</Dialog.Title>
        </Dialog.Header>
        <TextField onChange={setNewFolderName} value={newFolderName()}>
          <TextField.Label>Name</TextField.Label>
          <TextField.Input />
        </TextField>
        <Button onMouseDown={handleNewFolder} variant="success">
          Create
        </Button>
      </Dialog.Content>
    </Dialog>
  );
}

const columns: ColumnDef<Folder>[] = [
  {
    accessorKey: "name",
    header: () => <span>Name</span>,
    cell: (context) => context.getValue(),
    sortingFn: "text",
  },
];

function ProjectTable(props: { data: Folder[] }) {
  const table = createSolidTable({
    get data() {
      return props.data;
    },
    columns: columns,
    initialState: {
      sorting: [{ id: "name", desc: false }],
    },
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  });

  return (
    <Table>
      <Table.Header>
        <For each={table.getHeaderGroups()}>
          {(headerGroup) => (
            <Table.Row>
              <For each={headerGroup.headers}>
                {(header) => (
                  <Table.Head>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext(),
                        )}
                  </Table.Head>
                )}
              </For>
            </Table.Row>
          )}
        </For>
      </Table.Header>
      <Table.Body>
        <For each={table.getRowModel().rows}>
          {(row) => (
            <Table.Row>
              <For each={row.getVisibleCells()}>
                {(cell) => (
                  <Table.Cell>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </Table.Cell>
                )}
              </For>
            </Table.Row>
          )}
        </For>
      </Table.Body>
    </Table>
  );
}
