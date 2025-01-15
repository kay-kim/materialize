---
title: "ALTER ... SWAP"
description: "`ALTER ... SWAP` atomically renames two items."
menu:
  main:
    parent: 'commands'
---

`ALTER ... SWAP` atomically renames two items.

## Syntax

{{< diagram "alter-swap.svg" >}}

Field                | Use
---------------------|------------------------------------------------
_name_               | The identifier of the item you want to swap.
_target&lowbar;name_ | The target [identifier](/self-managed/v2025.01/sql/identifiers) of the item you want to swap with.

## Examples

Swapping two items is useful for a blue/green deployment

```mzsql
CREATE SCHEMA blue;
CREATE TABLE blue.numbers (n int);

CREATE SCHEMA green;
CREATE TABLE green.tags (tag text);

ALTER SCHEMA blue SWAP WITH green;

-- The schema which was previously named 'green' is now named 'blue'.
SELECT * FROM blue.tags;
```

## Privileges

The privileges required to execute this statement are:

- Ownership of both objects being swapped.

## See also

- [`SHOW CREATE VIEW`](/self-managed/v2025.01/sql/show-create-view)
- [`SHOW VIEWS`](/self-managed/v2025.01/sql/show-views)
- [`SHOW SOURCES`](/self-managed/v2025.01/sql/show-sources)
- [`SHOW INDEXES`](/self-managed/v2025.01/sql/show-indexes)
- [`SHOW SECRETS`](/self-managed/v2025.01/sql/show-secrets)
- [`SHOW SINKS`](/self-managed/v2025.01/sql/show-sinks)
