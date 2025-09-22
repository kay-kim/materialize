---
title: "CREATE SOURCE v2: PostgreSQL"
description: "Creates a new source from PostgreSQL 11+."
pagerank: 40
menu:
  main:
    parent: 'create-source-v2'
    name: "PostgreSQL"
---

{{< public-preview />}}

Creates a new source from PostgreSQL. Materialize supports creating sources from
PostgreSQL version 11+.

## Prerequisites

To create a source from PostgreSQL 11+, you need to first:

- Configure PostgreSQL for logical replication and create a
  [publication](https://www.postgresql.org/docs/current/logical-replication-publication.html)
  to be streamed to Materialize. For details, see the [integration guides for
  PostgreSQL](/ingest-data/postgres/#integration-guides).

- Create a replication user and password in PostgreSQL that Materialize will use
  to connect. For details, see the [integration guides for your
  PostgreSQL](/ingest-data/postgres/#integration-guides).

- Define in Materialize a [connection to your
  PostgreSQL](#creating-a-connection). Once created, a connection is
  **reusable** across multiple `CREATE SOURCE` statements.

{{< annotation type="Available integration guides" >}}

For step-by-step instructions on configuring PostgreSQL, see the integration
guide for your PostgreSQL:

{{< include-md file="shared-content/postgresql-ingest-data-guides.md" >}}

{{</ annotation >}}

## Syntax

{{% include-example file="examples/create_source/example_postgres_source"
 example="syntax" %}}

## Details

### Change data capture

Materialize uses PostgreSQL's native replication protocol to continually ingest
changes resulting from `INSERT`, `UPDATE` and `DELETE` operations in the
upstream database — a process also known as _change data capture_.

### PostgreSQL replication slots

When you define a source, Materialize will automatically create a **replication
slot** in the upstream PostgreSQL database (see [PostgreSQL replication
slots](#postgresql-replication-slots)). Each source ingests the raw replication
stream data for all tables in the specified publication using **a single**
replication slot. This allows you to minimize the performance impact on the
upstream database as well as reuse the same source across multiple
materializations.

The name of the replication slot created by Materialize is prefixed with
`materialize_`. In Materialize, you can query the
`mz_internal.mz_postgres_sources` to find the replication slots created:

```mzsql
SELECT id, replication_slot FROM mz_internal.mz_postgres_sources;
```

```
    id   |             replication_slot
---------+----------------------------------------------
  u8     | materialize_7f8a72d0bf2a4b6e9ebc4e61ba769b71
```


{{< note >}}
The schema metadata is captured when the source is
initially created and is validated against the upstream schema upon restart.
If you create new tables upstream after creating a PostgreSQL source and want to
replicate them to Materialize, the source must be dropped and recreated.
{{</ >}}

{{< tip >}}

- {{< include-md file="shared-content/postgres-wal.md" >}}

{{< include-md file="shared-content/postgres-remove-unused-replication-slots.md" >}}

{{</ tip >}}

### Ingesting data

After a source is created, you can create tables from the source, referencing
the tables in the publication, to start ingesting data. See [Create a table
(PostgreSQL source)](/sql/create-table/#create-a-table-postgresql-source) for
details. You can create multiple tables that reference the same table in the
publication.

## Known limitations

{{% include-md file="shared-content/postgres-known-limitations.md" %}}

## Examples

### Prerequisite: Configure upstream PostgreSQL

Before creating a source, you must:

- Set up logical replication in the upstream database.

- Create a publication.

- Create a replication user and password for Materialize to use to connect.

For step-by-step instructions, see the PostgreSQL integration guides:

{{< include-md file="shared-content/postgresql-ingest-data-guides.md" >}}

### Prerequisite: Create a connection

A connection details how to connect and authenticate Materialize to an external
system. Once created, a connection is **reusable** across multiple `CREATE
SOURCE` statements. For more details on creating connections, check the [`CREATE
CONNECTION`](/sql/create-connection/#postgresql) documentation page.

Create a connection using the replication user and password set up in the
upstream database.

{{< tabs tabID="1" >}}
{{< tab "Direct connection">}}


```mzsql
CREATE SECRET pgpass AS '<POSTGRES_PASSWORD>';

CREATE CONNECTION pg_connection TO POSTGRES (
    HOST '<POSTGRES_URL>',
    PORT 5432,
    USER '<POSTGRES_USER>',
    PASSWORD SECRET pgpass,
    SSL MODE 'required',
    DATABASE 'postgres'
);
```

{{< /tab >}}
{{< tab "AWS PrivateLink">}}

For step-by-step instructions on creating AWS PrivateLink connections and
configuring an AWS PrivateLink service to accept connections from Materialize,
check [this guide](/ops/network-security/privatelink/).

```mzsql
CREATE CONNECTION privatelink_svc TO AWS PRIVATELINK (
    SERVICE NAME 'com.amazonaws.vpce.us-east-1.vpce-svc-0e123abc123198abc',
    AVAILABILITY ZONES ('use1-az1', 'use1-az4')
);
```

```mzsql
CREATE SECRET pgpass AS '<POSTGRES_PASSWORD>';

CREATE CONNECTION pg_connection TO POSTGRES (
    HOST 'instance.foo000.us-west-1.rds.amazonaws.com',
    PORT 5432,
    USER 'postgres',
    PASSWORD SECRET pgpass,
    AWS PRIVATELINK privatelink_svc,
    DATABASE 'postgres'
);
```

{{< /tab >}}
{{< tab "SSH tunnel">}}

For step-by-step instructions on creating SSH tunnel connections and configuring
an SSH bastion server to accept connections from Materialize, check
[this guide](/ops/network-security/ssh-tunnel/).

```mzsql
CREATE CONNECTION ssh_connection TO SSH TUNNEL (
    HOST 'bastion-host',
    PORT 22,
    USER 'materialize',
);
```

```mzsql
CREATE CONNECTION pg_connection TO POSTGRES (
    HOST 'instance.foo000.us-west-1.rds.amazonaws.com',
    PORT 5432,
    SSH TUNNEL ssh_connection,
    DATABASE 'postgres'
);
```



{{< /tab >}}
{{< /tabs >}}

### Create a source {#create-source-example}

{{% include-example file="examples/create_source/example_postgres_source"
 example="create-source" %}}

{{% include-example file="examples/create_source/example_postgres_source"
 example="create-table" %}}

#### Adding subsources

When adding tables to a PostgreSQL source, Materialize opens a temporary
replication slot to snapshot the new table's current states. After
completing the snapshot, the table will be kept up-to-date, like all other
tables in the publication.

#### Dropping subsources

Dropping a table in PostgreSQL prevents Materialize from ingesting any data from
it, in addition to dropping any state that Materialize previously had for the
table.

## Related pages

- [`CREATE SECRET`](/sql/create-secret)
- [`CREATE CONNECTION`](/sql/create-connection)
- [`CREATE SOURCE`](../)
- PostgreSQL integration guides:
  - [AlloyDB](/ingest-data/postgres-alloydb/)
  - [Amazon RDS](/ingest-data/postgres-amazon-rds/)
  - [Amazon Aurora](/ingest-data/postgres-amazon-aurora/)
  - [Azure DB](/ingest-data/postgres-azure-db/)
  - [Google Cloud SQL](/ingest-data/postgres-google-cloud-sql/)
  - [Self-hosted](/ingest-data/postgres-self-hosted/)

[`enum`]: https://www.postgresql.org/docs/current/datatype-enum.html
[`money`]: https://www.postgresql.org/docs/current/datatype-money.html
