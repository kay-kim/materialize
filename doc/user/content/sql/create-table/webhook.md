---
title: "CREATE TABLE: Webhook"
description: "Create a read-only table in Materialize that is populated by webhook events."
pagerank: 50
menu:
  main:
    parent: 'create-table'
    identifier: 'create-table-webhook'
    name: "Webhook table"
    weight: 60
---

In Materialize, you can create webhook-populated tables. Webhook-populated
tables cannot be written to by the user; they are read-only. These tables are
populated through data posted to the associated **public** webhook URL, which is
automatically created with the table creation.

## Syntax

{{% include-syntax file="examples/create_table_webhook" example="syntax" %}}

## Details

### Webhook URL

After the table creation, the unique URL that allows you to **POST** events to
the table can be looked up in the
[`mz_internal.mz_webhook_sources`](/reference/system-catalog/mz_internal/#mz_webhook_sources)
system catalog table. The URL will have the following format:

```
https://<HOST>/api/webhook/<database>/<schema>/<table_name>
```
{{< warning >}}
This is a public URL that is open to the internet and has no security. To
validate that requests are legitimate, see [Validating requests](#validating-requests).
For limits imposed on this endpoint, see [Request limits](#request-limits).
{{< /warning >}}

### Ingesting data

Webhook data is ingested as a JSON blob. We recommend creating a parsing view on
top of your webhook table that maps the individual fields to columns with the
required data types. You can use [this **JSON parsing
widget**](/sql/types/jsonb/#parsing) to avoid parsing manually.

## Request limits

Webhook tables apply the following limits to received requests:

* The maximum size of the request body is **`2MB`**. Requests larger than this
  will fail with `413 Payload Too Large`.
* The maximum number of concurrent requests across **all** webhooks is **500**.
  Trying to connect when the server is at capacity will fail with
  `429 Too Many Requests`.
* Requests that contain a header name specified more than once will be rejected
  with `401 Unauthorized`.

## Examples

### Creating a webhook table

{{% include-example file="examples/create_table_webhook"
 example="create-table" %}}

{{% include-example file="examples/create_table_webhook"
 example="post-to-webhook" %}}

{{% include-example file="examples/create_table_webhook"
 example="read-from-table" %}}

{{% include-example file="examples/create_table_webhook"
 example="create-a-view-from-table" %}}

### Exposing headers

{{% include-example file="examples/create_table_webhook"
 example="exposing-headers" %}}

#### Excluding header fields

{{% include-example file="examples/create_table_webhook"
 example="excluding-header-fields" %}}

### Validating requests

{{< warning >}}
Without a `CHECK` statement, **all requests will be accepted**. To prevent bad
actors from injecting data into your table, it is **strongly encouraged** that
you define a `CHECK` statement with your webhook tables.
{{< /warning >}}

{{% include-example file="examples/create_table_webhook"
 example="validating-requests" %}}

#### Debugging validation

{{% include-example file="examples/create_table_webhook"
 example="debugging-validation" %}}

{{% include-example file="examples/create_table_webhook"
 example="debugging-validation-query" %}}

### Handling duplicated and partial events

{{% include-example file="examples/create_table_webhook"
 example="handling-duplicated-events" %}}

{{% include-example file="examples/create_table_webhook"
 example="handling-partial-events" %}}

### Handling batch events

The application pushing events to your webhook table may batch multiple events
into a single HTTP request. Webhook tables support parsing batched events
in the following formats:

#### JSON arrays

{{% include-example file="examples/create_table_webhook"
 example="batch-json-array" %}}

{{% include-example file="examples/create_table_webhook"
 example="batch-json-array-count" %}}

{{% include-example file="examples/create_table_webhook"
 example="batch-json-array-single" %}}

#### Newline-delimited JSON (NDJSON)

{{% include-example file="examples/create_table_webhook"
 example="batch-ndjson" %}}

{{% include-example file="examples/create_table_webhook"
 example="batch-ndjson-count" %}}

## Related pages

- [`CREATE TABLE`](../)
- [`CREATE SECRET`](/sql/create-secret)
- [`CREATE SOURCE: Webhook (Legacy)`](/sql/create-source/webhook/)
- [`DROP TABLE`](/sql/drop-table/)
