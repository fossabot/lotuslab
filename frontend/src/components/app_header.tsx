// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { useNavigate } from "@solidjs/router";
import { IconFlaskConical, IconHome, IconSearch, IconSettings } from "./icons";
import { Button } from "./ui/button";

export function AppHeader() {
  const navigate = useNavigate();

  return (
    <nav>
      <ul class="grid w-full grid-cols-3 items-center p-2">
        <div class="inline-flex items-center gap-2 justify-self-start">
          <li>
            <Button
              onMouseDown={() => navigate("/", { replace: true })}
              size="icon"
              variant="neutral_skeleton"
            >
              <IconHome class="size-6" />
            </Button>
          </li>
          <li>
            <Button onMouseDown={() => navigate("/theme_preview")}>
              Theme Preview
            </Button>
          </li>
        </div>
        <li class="justify-self-center">
          <Button onMouseDown={() => navigate("/search", { replace: true })}>
            <IconSearch class="size-4" />
            Search all cards
          </Button>
        </li>
        <div class="inline-flex items-center gap-2 justify-self-end">
          <li>
            <Button
              onMouseDown={() => navigate("/testing", { replace: true })}
              size="icon"
              variant="neutral_skeleton"
            >
              <IconFlaskConical class="size-6" />
            </Button>
          </li>
          <li>
            <Button
              onMouseDown={() => navigate("/settings", { replace: true })}
              size="icon"
              variant="neutral_skeleton"
            >
              <IconSettings class="size-6" />
            </Button>
          </li>
        </div>
      </ul>
    </nav>
  );
}
