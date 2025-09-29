---
title: "CREATE TABLE: Kafka/Redpanda"
description: "Reference page for `CREATE TABLE`. `CREATE TABLE` creates a table that is persisted in durable storage."
menu:
  # This should also have a "non-content entry" under Reference, which is
  # configured in doc/user/config.toml
  main:
    parent: 'create-table'
    name: "Kafka/Redpanda"
    identifier: 'create-table-kafka'
---

{{< source-versioning-disambiguation is_new=true other_ref="[old reference page](/sql/create-source-v1/kafka/)" >}}

`CREATE TABLE` defines a table that is persisted in durable storage. In
Materialize, you can create tables populated from Kafka sources.
Source-populated tables are read-only tables; they cannot be written to by the
user. These tables are populated by [data ingestion from a source](/ingest-data/).

Tables in Materialize are similar to tables in standard relational databases:
they consist of rows and columns where the columns are fixed when the table is
created.

Tables can be joined with other tables, materialized views, and views; and you
can create views/materialized views/indexes on tables.

## Syntax

Use the format-specific syntax to create a table from a Kafka/Redpanda source:

{{< note >}}

- {{< include-md file="shared-content/kafka-redpanda-shorthand.md" >}}

- {{< include-md file="shared-content/create-table-from-source-readonly.md" >}}

- {{< include-md file="shared-content/create-table-from-source-snapshotting.md"
  >}}

{{</ note >}}

### FORMAT AVRO

{{% include-example file="examples/create_table/example_kafka_table_avro"
 example="syntax" %}}

### FORMAT JSON

{{% include-example file="examples/create_table/example_kafka_table_json"
 example="syntax" %}}

### FORMAT TEXT/BYTES

{{% include-example file="examples/create_table/example_kafka_table_text"
 example="syntax" %}}

### FORMAT CSV

{{% include-example file="examples/create_table/example_kafka_table_csv"
 example="syntax" %}}

### KEY FORMAT VALUE FORMAT

{{% include-example file="examples/create_table/example_kafka_table_key_value"
 example="syntax" %}}

## Details

### Table names and column names

Names for tables and column(s) must follow the [naming
guidelines](/sql/identifiers/#naming-restrictions).

<a name="supported-db-source-types"></a>


### Source-populated tables and snapshotting

{{< include-md file="shared-content/create-table-from-source-snapshotting.md"
>}}

### Required privileges

The privileges required to execute this statement are:

{{< include-md file="shared-content/sql-command-privileges/create-table.md" >}}

### Handling upserts

To create a source that uses the standard key-value convention to support
inserts, updates, and deletes within Materialize, you can use `ENVELOPE
UPSERT`:


{{< note >}}

- Using this envelope is required to consume [log compacted topics](https://docs.confluent.io/platform/current/kafka/design.html#log-compaction).

- This envelope can lead to high memory and disk utilization in the cluster
  maintaining the source. We recommend using a standard-sized cluster, rather
  than a legacy-sized cluster, to automatically spill the workload to disk. See
  [spilling to disk](#spilling-to-disk) for details.

{{< /note >}}

#### Null keys

If a message with a `NULL` key is detected, Materialize sets the source into an
error state. To recover an errored source, you must produce a record with a
`NULL` value and a `NULL` key to the topic, to force a retraction.

As an example, you can use [`kcat`](https://docs.confluent.io/platform/current/clients/kafkacat-usage.html)
to produce an empty message:

```bash
echo ":" | kcat -b $BROKER -t $TOPIC -Z -K: \
  -X security.protocol=SASL_SSL \
  -X sasl.mechanisms=SCRAM-SHA-256 \
  -X sasl.username=$KAFKA_USERNAME \
  -X sasl.password=$KAFKA_PASSWORD
```

#### Value decoding errors

By default, if an error happens while decoding the value of a message for a
specific key, Materialize sets the source into an error state. You can
configure the source to continue ingesting data in the presence of value
decoding errors using the `VALUE DECODING ERRORS = INLINE` option:

```mzsql
CREATE SOURCE kafka_upsert
  FROM KAFKA CONNECTION kafka_connection (TOPIC 'events')
  KEY FORMAT AVRO USING CONFLUENT SCHEMA REGISTRY CONNECTION csr_connection
  VALUE FORMAT AVRO USING CONFLUENT SCHEMA REGISTRY CONNECTION csr_connection
  ENVELOPE UPSERT (VALUE DECODING ERRORS = INLINE);
```

When this option is specified the source will include an additional column named
`error` with type `record(description: text)`.

This column and all value columns will be nullable, such that if the most recent value
for the given Kafka message key cannot be decoded, this `error` column will contain
the error message. If the most recent value for a key has been successfully decoded,
this column will be `NULL`.

To use an alternative name for the error column, use `INLINE AS ..` to specify the
column name to use:

```mzsql
ENVELOPE UPSERT (VALUE DECODING ERRORS = (INLINE AS my_error_col))
```

It might be convenient to implement a parsing view on top of your Kafka upsert source that
excludes keys with decoding errors:

```mzsql
CREATE VIEW kafka_upsert_parsed
SELECT *
FROM kafka_upsert
WHERE error IS NULL;
```

### Using Debezium

{{< debezium-json >}}

Materialize provides a dedicated envelope (`ENVELOPE DEBEZIUM`) to decode Kafka
messages produced by [Debezium](https://debezium.io/). To create a source that
interprets Debezium messages:

```mzsql
CREATE SOURCE kafka_repl
  FROM KAFKA CONNECTION kafka_connection (TOPIC 'pg_repl.public.table1')
  FORMAT AVRO USING CONFLUENT SCHEMA REGISTRY CONNECTION csr_connection
  ENVELOPE DEBEZIUM;
```

Any materialized view defined on top of this source will be incrementally
updated as new change events stream in through Kafka, as a result of `INSERT`,
`UPDATE` and `DELETE` operations in the original database.

For more details and a step-by-step guide on using Kafka+Debezium for Change
Data Capture (CDC), check [Using Debezium](/integrations/debezium/).

Note that:

- This envelope can lead to high memory utilization in the cluster maintaining
  the source. Materialize can automatically offload processing to
  disk as needed. See [spilling to disk](#spilling-to-disk) for details.

### Spilling to disk

Kafka sources that use `ENVELOPE UPSERT` or `ENVELOPE DEBEZIUM` require storing
the current value for _each key_ in the source to produce retractions when keys
are updated. When using [standard cluster sizes](/sql/create-cluster/#size),
Materialize will automatically offload this state to disk, seamlessly handling
key spaces that are larger than memory.

Spilling to disk is not available with [legacy cluster sizes](/sql/create-cluster/#legacy-sizes).

### Exposing source metadata

In addition to the message value, Materialize can expose the message key,
headers and other source metadata fields to SQL.

#### Key

The message key is exposed via the `INCLUDE KEY` option. Composite keys are also
supported.

```mzsql
CREATE SOURCE kafka_metadata
  FROM KAFKA CONNECTION kafka_connection (TOPIC 'data')
  KEY FORMAT TEXT
  VALUE FORMAT TEXT
  INCLUDE KEY AS renamed_id;
```

Note that:

- This option requires specifying the key and value encodings explicitly using the `KEY FORMAT ... VALUE FORMAT` [syntax](#syntax).

- The `UPSERT` envelope always includes keys.

- The `DEBEZIUM` envelope is incompatible with this option.

#### Headers

Message headers can be retained in Materialize and exposed as part of the source data.

Note that:
- The `DEBEZIUM` envelope is incompatible with this option.

**All headers**

All of a message's headers can be exposed using `INCLUDE HEADERS`, followed by
an `AS <header_col>`.

This introduces column with the name specified or `headers` if none was
specified. The column has the type `record(key: text, value: bytea?) list`,
i.e. a list of records containing key-value pairs, where the keys are `text`
and the values are nullable `bytea`s.

```mzsql
CREATE SOURCE kafka_metadata
  FROM KAFKA CONNECTION kafka_connection (TOPIC 'data')
  FORMAT AVRO USING CONFLUENT SCHEMA REGISTRY CONNECTION csr_connection
  INCLUDE HEADERS
  ENVELOPE NONE;
```

To simplify turning the headers column into a `map` (so individual headers can
be searched), you can use the [`map_build`](/sql/functions/#map_build) function:

```mzsql
SELECT
    id,
    seller,
    item,
    convert_from(map_build(headers)->'client_id', 'utf-8') AS client_id,
    map_build(headers)->'encryption_key' AS encryption_key,
FROM kafka_metadata;
```

<p></p>

```nofmt
 id | seller |        item        | client_id |    encryption_key
----+--------+--------------------+-----------+----------------------
  2 |   1592 | Custom Art         |        23 | \x796f75207769736821
  3 |   1411 | City Bar Crawl     |        42 | \x796f75207769736821
```

**Individual headers**

Individual message headers can be exposed via the `INCLUDE HEADER key AS name`
option.

The `bytea` value of the header is automatically parsed into an UTF-8 string. To
expose the raw `bytea` instead, the `BYTES` option can be used.

```mzsql
CREATE SOURCE kafka_metadata
  FROM KAFKA CONNECTION kafka_connection (TOPIC 'data')
  FORMAT AVRO USING CONFLUENT SCHEMA REGISTRY CONNECTION csr_connection
  INCLUDE HEADER 'c_id' AS client_id, HEADER 'key' AS encryption_key BYTES,
  ENVELOPE NONE;
```

Headers can be queried as any other column in the source:

```mzsql
SELECT
    id,
    seller,
    item,
    client_id::numeric,
    encryption_key
FROM kafka_metadata;
```

<p></p>

```nofmt
 id | seller |        item        | client_id |    encryption_key
----+--------+--------------------+-----------+----------------------
  2 |   1592 | Custom Art         |        23 | \x796f75207769736821
  3 |   1411 | City Bar Crawl     |        42 | \x796f75207769736821
```

Note that:

- Messages that do not contain all header keys as specified in the source DDL
  will cause an error that prevents further querying the source.

- Header values containing badly formed UTF-8 strings will cause an error in the
  source that prevents querying it, unless the `BYTES` option is specified.

#### Partition, offset, timestamp

These metadata fields are exposed via the `INCLUDE PARTITION`, `INCLUDE OFFSET`
and `INCLUDE TIMESTAMP` options.

```mzsql
CREATE SOURCE kafka_metadata
  FROM KAFKA CONNECTION kafka_connection (TOPIC 'data')
  FORMAT AVRO USING CONFLUENT SCHEMA REGISTRY CONNECTION csr_connection
  INCLUDE PARTITION, OFFSET, TIMESTAMP AS ts
  ENVELOPE NONE;
```

```mzsql
SELECT "offset" FROM kafka_metadata WHERE ts > '2021-01-01';
```

<p></p>

```nofmt
offset
------
15
14
13
```

## Examples

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

## Related pages

- [`INSERT`](../insert)
- [`CREATE SOURCE`](/sql/create-source/)
- [`DROP TABLE`](../drop-table)
- [Ingest data](/ingest-data/)

[`INSERT`]: /sql/insert/
[`SELECT`]: /sql/select/
[`UPDATE`]: /sql/update/
[`DELETE`]: /sql/delete/
