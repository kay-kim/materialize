---
title: "CREATE SOURCE (v2)"
description: "Private Preview: CREATE SOURCE reference page"
disable_list: true
menu:
  main:
    parent: 'commands'
    identifier: 'create-source-v2'
---

Creates a new source that connects to an upstream/external system.

{{< public-preview />}}

## Syntax yummary

Materialize can ingest data from the following external systems. For the
specific syntax, refer to the specific `CREATE SOURCE` (v2) pages.

{{< tabs >}}
{{< tab "PostgreSQL" >}}

{{% include-example file="examples/create_source/example_postgres_source"
 example="syntax" %}}

 For details, see [`CREATE SOURCE:
 PostgreSQL`](/sql/create-source-v2/postgres/).
 
 {{< /tab >}}
 {{< /tabs >}}

## Considerations

### Sources and clusters

For the sources, you need to associate a cluster to provide the compute
resources needed to ingest data.

{{< tip >}}

If possible, dedicate a cluster just for sources.

{{</ tip >}}
