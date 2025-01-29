-- SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
--
-- SPDX-License-Identifier: GPL-3.0-or-later

-- tables with basic fields
DEFINE TABLE folder SCHEMAFULL;
    DEFINE FIELD name   ON folder TYPE string;

DEFINE TABLE project SCHEMAFULL;
    DEFINE FIELD name   ON project TYPE string;

DEFINE TABLE list SCHEMAFULL;
    DEFINE FIELD name   ON list TYPE string;

-- DEFINE TABLE card SCHEMAFULL;
--     DEFINE FIELD name       ON card TYPE string;
--     DEFINE FIELD colors     ON card TYPE set<string>;
--     DEFINE FIELD mana_cost  ON card TYPE option<string>;
--     DEFINE FIELD text       ON card TYPE option<string>;

-- DEFINE TABLE tag SCHEMAFULL;
--     DEFINE FIELD name   ON tag TYPE string;
--     DEFINE FIELD color  ON tag TYPE option<string>;

-- relationships
-- DEFINE TABLE list_contains SCHEMAFULL TYPE RELATION FROM list TO card;
--     DEFINE FIELD added_at           ON list_contains DEFAULT time::now();
--     DEFINE FIELD updated_at         ON list_contains DEFAULT time::now();
--     DEFINE FIELD custom_image_uri   ON list_contains TYPE option<string>;

DEFINE FIELD parent     ON folder TYPE option<record<folder>> REFERENCE;
-- DEFINE FIELD children   ON folder COMPUTED array::concat(<~folder, <~project);

DEFINE FIELD folder     ON project TYPE record<folder> REFERENCE;
-- DEFINE FIELD lists      ON project COMPUTED <~list;

DEFINE FIELD project    ON list TYPE record<project> REFERENCE;

CREATE folder:root SET name = "root"
