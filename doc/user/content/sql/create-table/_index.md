---
title: "CREATE TABLE"
description: "CREATE TABLE reference page"
disable_list: true
menu:
  main:
    parent: 'commands'
    identifier: 'create-table'
---

{{< source-versioning-disambiguation is_new=true other_ref="[old reference page](/sql/create-table-v1/)" >}}

`CREATE TABLE` defines a table that is persisted in durable storage.

In Materialize, you can create:
- User-populated tables. User-populated tables are read-write tables; i.e.,
  users can read and write to the tables ([`INSERT`], [`SELECT`], [`UPDATE`],
  [`DELETE`]).

- [Source-populated](/concepts/sources/) tables. Source-populated tables are
  read-only tables; they cannot be written to by the user. These tables are
  populated by [data ingestion from a source](/ingest-data/).

- Webhook-populated tables. Webhook-populated tables cannot be written to by the
  user; they are read-only. These tables are populated through data posted to
  the associated **public** webhook URL, which is automatically created with the
  table creation.

Tables in Materialize are similar to tables in standard relational databases:
they consist of rows and columns where the columns are fixed when the table is
created.

Tables can be joined with other tables, materialized views, and views; and you
can create views/materialized views/indexes on tables.
