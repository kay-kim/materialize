---
title: "CREATE TABLE"
description: "Reference page for `CREATE TABLE`. `CREATE TABLE` creates a table that is persisted in durable storage."
pagerank: 40
menu:
  # This should also have a "non-content entry" under Reference, which is
  # configured in doc/user/config.toml
  main:
    parent: 'commands'
---

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

## Syntax

{{< tabs >}}

{{< tab "User-populated tables" >}}

{{% include-example file="examples/create_table/example_user_populated_table"
 example="syntax" %}}

{{</ tab >}}

{{< tab "Source-populated tables (via DB connector)" >}}

[//]: # "May not be able to make the syntax consistent across DB sources since
    we can't support SQL Server's text type. But if the others are consistent,
    can rework so that we don't need to have tabbed versions of the syntax per db"

Materialize provides native connectors for the following databases:

- MySQL
- PostgreSQL
- SQL Server

{{< tabs >}}
{{< tab "MySQL" >}}
{{% include-example file="examples/create_table/example_mysql_table"
 example="syntax" %}}
{{</ tab >}}
{{< tab "PostgreSQL" >}}
{{% include-example file="examples/create_table/example_postgres_table"
 example="syntax" %}}
{{</ tab >}}
{{< tab "SQL Server" >}}
{{% include-example file="examples/create_table/example_sqlserver_table"
 example="syntax" %}}
{{</ tab >}}
{{</ tabs >}}

{{</ tab >}}

{{< tab "Source-populated tables (connected via Kafka/Redpanda)" >}}

Use the format-specific syntax to create a table from a Kafka/Redpanda source:

{{< note >}}

- {{< include-md file="shared-content/kafka-redpanda-shorthand.md" >}}

- {{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

- {{< include-md file="shared-content/create-table-from-source-snapshotting.md"
  >}}

{{</ note >}}


{{< tabs >}}

{{< tab "FORMAT AVRO" >}}

{{% include-example file="examples/create_table/example_kafka_table_avro"
 example="syntax" %}}

{{</ tab >}}

{{< tab "FORMAT JSON" >}}

{{% include-example file="examples/create_table/example_kafka_table_json"
 example="syntax" %}}

{{</ tab >}}

{{< tab "FORMAT TEXT/BYTES" >}}

{{% include-example file="examples/create_table/example_kafka_table_text"
 example="syntax" %}}

{{</ tab >}}

{{< tab "FORMAT CSV" >}}

{{% include-example file="examples/create_table/example_kafka_table_csv"
 example="syntax" %}}
{{</ tab >}}

{{< tab "KEY FORMAT VALUE FORMAT" >}}

{{% include-example file="examples/create_table/example_kafka_table_key_value"
 example="syntax" %}}

{{</ tab >}}
{{</ tabs >}}

{{</ tab >}}

{{< tab "Webhook-populated table" >}}

{{% include-example file="examples/create_table/example_kafka_table_webhook"
 example="syntax" %}}

{{</ tab >}}

{{</ tabs >}}

## Details

### Table names and column names

Names for tables and column(s) must follow the [naming
guidelines](/sql/identifiers/#naming-restrictions).

<a name="supported-db-source-types"></a>

### Upstream sources and supported data types

{{< include-md file="shared-content/create-table-supported-types.md" >}}

### Source-populated tables and snapshotting

{{< include-md file="shared-content/create-table-from-source-snapshotting.md"
>}}

### Known limitations

Tables do not currently support:

- Primary keys
- Unique constraints
- Check constraints

See also the known limitations for [`INSERT`](../insert#known-limitations),
[`UPDATE`](../update#known-limitations), and [`DELETE`](../delete#known-limitations).

### Required privileges

The privileges required to execute this statement are:

{{< include-md file="shared-content/sql-command-privileges/create-table.md" >}}

## Examples

### Create a table (User-populated)

{{% include-example file="examples/create_table/example_user_populated_table"
 example="create-table" %}}

Once a user-populated table is created, you can perform CRUD
(Create/Read/Update/Write) operations on it.

{{% include-example file="examples/create_table/example_user_populated_table"
 example="write-to-table" %}}

{{% include-example file="examples/create_table/example_user_populated_table"
 example="read-from-table" %}}

### Create a table (PostgreSQL Source)

{{< note >}}

The example assumes you have configured your upstream PostgreSQL 11+ (i.e.,
enabled logical replication, created the publication for the various tables and
replication user, and updated the network configuration).

For details about configuring your upstream system, see the [PostgreSQL
integration guides](/ingest-data/postgres/#supported-versions-and-services).

{{</ note >}}

{{% include-example file="examples/create_table/example_postgres_table"
 example="create-table" %}}

{{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

{{< include-md file="shared-content/create-table-from-source-snapshotting.md"
>}}

Once the snapshotting process completes, you can query the table:

{{% include-example file="examples/create_table/example_postgres_table"
 example="read-from-table" %}}

### Create a table (MySQL Source)

{{< note >}}

The example assumes you have configured your upstream MySQL 5.7+ (i.e.,
enabled GTID-based binlog replication, created the
replication user, and updated the network configuration as needed).

For details about configuring your upstream system, see the [MySQL
integration guides](/ingest-data/mysql/#supported-versions-and-services).

{{</ note >}}

{{% include-example file="examples/create_table/example_mysql_table"
 example="create-table" %}}

{{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

{{< include-md file="shared-content/create-table-from-source-snapshotting.md"
>}}

Once the snapshotting process completes, you can query the table:

{{% include-example file="examples/create_table/example_mysql_table"
 example="read-from-table" %}}

### Create a table (SQL Server Source)

{{< note >}}

The example assumes you have configured your upstream SQL Server 2016+ (i.e.,
created the replication user, enabled change data capture and `SNAPSHOT`
transaction isolation,, and updated the network configuration as needed).

For details about configuring your upstream system, see the [SQL Server
integration guide](/ingest-data/sql-server/self-hosted/).

{{</ note >}}

{{% include-example file="examples/create_table/example_sqlserver_table"
 example="create-table" %}}

{{< include-md file="shared-content/create-table-from-source-readonly.md" >}}
{{< include-md file="shared-content/create-table-from-source-snapshotting.md"
>}}

{{< note >}}
{{< include-md file="shared-content/sql-server-snapshot-latency.md" >}}
{{</ note >}}

Once the snapshotting process completes, you can query the table:

{{% include-example file="examples/create_table/example_sqlserver_table"
 example="read-from-table" %}}


### Create a table (Kafka Source: Format AVRO)

{{< tip >}}
The same syntax may be used for Redpanda.
{{</ tip >}}

{{% include-example file="examples/create_table/example_kafka_table_avro"
 example="create-table" %}}

{{% include-example file="examples/create_table/example_kafka_table_avro"
 example="read-from-table" %}}

{{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

### Create a table (Kafka Source: Format JSON)

{{% include-example file="examples/create_table/example_kafka_table_json"
 example="create-table" %}}

{{% include-example file="examples/create_table/example_kafka_table_json"
 example="read-from-table" %}}

{{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

{{% include-example file="examples/create_table/example_kafka_table_json"
 example="create-a-view-from-table" %}}

### Create a table (Kafka Source: Format TEXT)

{{% include-example file="examples/create_table/example_kafka_table_text"
 example="create-table" %}}

{{% include-example file="examples/create_table/example_kafka_table_text"
 example="read-from-table" %}}

{{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

### Create a table (Kafka Source: Format CSV)

{{% include-example file="examples/create_table/example_kafka_table_csv"
 example="create-table" %}}

{{% include-example file="examples/create_table/example_kafka_table_csv"
 example="read-from-table" %}}

{{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

{{% include-example file="examples/create_table/example_kafka_table_csv"
 example="create-table-with-colnames" %}}

{{% include-example file="examples/create_table/example_kafka_table_csv"
 example="read-from-table-with-colnames" %}}

### Create a table (Kafka Source: Key Format Value Format)

{{% include-example file="examples/create_table/example_kafka_table_key_value"
 example="create-table" %}}

{{% include-example file="examples/create_table/example_kafka_table_key_value"
 example="read-from-table" %}}

{{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

{{% include-example file="examples/create_table/example_kafka_table_key_value"
 example="create-a-view-from-table" %}}

### Create a table (Webhook-populated)

{{% include-example file="examples/create_table/example_kafka_table_webhook"
 example="create-table" %}}

{{% include-example file="examples/create_table/example_kafka_table_webhook"
 example="post-to-webhook" %}}

{{% include-example file="examples/create_table/example_kafka_table_webhook"
 example="read-from-table" %}}

{{% include-example file="examples/create_table/example_kafka_table_webhook"
 example="create-a-view-from-table" %}}

## Related pages

- [`INSERT`](../insert)
- [`CREATE SOURCE`](/sql/create-source/)
- [`DROP TABLE`](../drop-table)
- [Ingest data](/ingest-data/)

[`INSERT`]: /sql/insert/
[`SELECT`]: /sql/select/
[`UPDATE`]: /sql/update/
[`DELETE`]: /sql/delete/
