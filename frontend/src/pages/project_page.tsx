// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { createSignal, Show, Suspense } from "solid-js";
import { IconPencil } from "../components/icons.tsx";
import { Button } from "../components/ui/button";
import { Dialog } from "../components/ui/dialog";
import { TextField } from "../components/ui/text-field";

export function ProjectPage() {
  const EditProjectDialog = () => {
    const [project_settings_dialog_open, set_project_settings_dialog_open] =
      createSignal(false);
    const [new_project_name, set_new_project_name] = createSignal("");
    const [new_project_description, set_new_project_description] =
      createSignal("");

    // When dialog opens, default the signals to current values
    function handle_open() {
      set_project_settings_dialog_open(true);
    }

    const handle_edit_project = async () => {
      set_project_settings_dialog_open(false);
      set_new_project_name("");
      set_new_project_description("");
    };

    return (
      <Dialog
        onOpenChange={set_project_settings_dialog_open}
        open={project_settings_dialog_open()}
      >
        <Button onMouseDown={handle_open} size="icon" variant="success">
          <IconPencil />
        </Button>
        <Dialog.Content
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              handle_edit_project();
            }
          }}
        >
          <Dialog.CloseButtonX
            onMouseDown={() => set_project_settings_dialog_open(false)}
          />
          <Dialog.Header>
            <Dialog.Title>Edit Project</Dialog.Title>
          </Dialog.Header>
          <div class="flex flex-col items-stretch justify-center gap-4">
            <TextField
              onChange={set_new_project_name}
              value={new_project_name()}
            >
              <TextField.Label>Name</TextField.Label>
              <TextField.Input />
            </TextField>
            <TextField
              onChange={set_new_project_description}
              value={new_project_description()}
            >
              <TextField.Label>Description</TextField.Label>
              <TextField.TextArea />
            </TextField>
            <Button
              onKeyDown={(e: KeyboardEvent) => {
                if (e.key === "Enter" || e.key === " ") {
                  handle_edit_project();
                }
              }}
              onMouseDown={handle_edit_project}
              variant="success"
            >
              Submit
            </Button>
          </div>
        </Dialog.Content>
      </Dialog>
    );
  };

  return (
    <main class="mx-auto flex w-[80%] flex-col pt-10">
      <Suspense>
        <div class="mb-margin flex items-baseline justify-between">
          <h1 class="text-4xl leading-tight">Project Name</h1>
          <EditProjectDialog />
        </div>
        <Show when={true}>
          {(_) => (
            <>
              <h2 class="font-bold">Description</h2>
              <p>This is a description</p>
            </>
          )}
        </Show>
      </Suspense>
    </main>
  );
}
