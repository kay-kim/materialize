---
title: "CREATE SOURCE"
description: "CREATE SOURCE reference page"
disable_list: true
menu:
  main:
    parent: 'commands'
    identifier: 'create-source'
---

{{< source-versioning-disambiguation is_new=true other_ref="[old reference page](/sql/create-source-v1/)" >}}

Creates a new source that connects to an upstream/external system.

## Syntax summary

Materialize can ingest data from the following external systems.

{{< tabs >}}
{{< tab "Kafka" >}}
{{% include-example file="examples/create_source/example_kafka_source"
 example="syntax" %}}

{{< /tab >}}
{{< tab "MySQL" >}}

{{% include-example file="examples/create_source/example_mysql_source"
 example="syntax" %}}

 {{< /tab >}}
{{< tab "PostgreSQL" >}}

{{% include-example file="examples/create_source/example_postgres_source"
 example="syntax" %}}

 {{< /tab >}}

 {{< tab "SQL Server" >}}

{{% include-example file="examples/create_source/example_sql_server_source"
 example="syntax" %}}

 {{< /tab >}}
 {{< /tabs >}}

For details, refer to the specific `CREATE SOURCE` pages:
- [CREATE SOURCE: Kafka](/sql/create-source/Kafka/)
- [CREATE SOURCE: MySQL](/sql/create-source/mysql/)
- [CREATE SOURCE: PostgreSQL](/sql/create-source/postgres/)
- [CREATE SOURCE: SQL Server](/sql/create-source/sql-server/)

## Considerations

### Sources and clusters

For the sources, you need to associate a cluster to provide the compute
resources needed to ingest data.

{{< tip >}}

If possible, dedicate a cluster just for sources.

{{</ tip >}}
