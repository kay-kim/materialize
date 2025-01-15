---
title: "Overview"
description: "Learn how to efficiently transform data using Materialize SQL."
disable_list: true
menu:
  main:
    parent: transform-data
    weight: 5
    identifier: transform-overview
---

With Materialize, you can use SQL to transform, deliver, and act on
fast-changing data.

{{% materialize-postgres-compatibility %}}

### SELECT statement

To build your transformations, you can [`SELECT`](/self-managed/v2025.01/sql/select/) from
[sources](/self-managed/v2025.01/concepts/sources/), tables, [views](/self-managed/v2025.01/concepts/views/#views), and
[materialized views](/self-managed/v2025.01/concepts/views/#materialized-views).

```mzsql
SELECT [ ALL | DISTINCT [ ON ( col_ref [, ...] ) ] ]
    [ { * | projection_expr [ [ AS ] output_name ] } [, ...] ]
    [ FROM table_expr [ join_expr | , ] ... ]
    [ WHERE condition_expr ]
    [ GROUP BY grouping_expr [, ...] ]
    [ OPTIONS ( option = val[, ...] ) ]
    [ HAVING having_expr ]
    [ ORDER BY projection_expr [ ASC | DESC ] [ NULLS FIRST | NULLS LAST ] [, ...] ]
    [ LIMIT { integer  } [ OFFSET { integer } ] ]
    [ { UNION | INTERSECT | EXCEPT } [ ALL | DISTINCT ] { SELECT ...} ]
```

In Materialize, the [`SELECT`](/self-managed/v2025.01/sql/select/) statement supports (among others):

- [JOINS (inner, left outer, right outer, full outer,
  cross)](/self-managed/v2025.01/sql/select/join/) and [lateral
  subqueries](/self-managed/v2025.01/sql/select/join/#lateral-subqueries)

- [Common Table Expressions (CTEs)](/self-managed/v2025.01/sql/select/#common-table-expressions-ctes)
  and [Recursive CTEs](/self-managed/v2025.01/sql/select/recursive-ctes/)

- [Query hints (`AGGREGATE INPUT GROUP SIZE`, `DISTINCT ON INPUT GROUP SIZE`,
  `LIMIT INPUT GROUP SIZE`)](/self-managed/v2025.01/sql/select/#query-hints)

- [SQL functions](/self-managed/v2025.01/sql/functions/) and [operators](/self-managed/v2025.01/sql/functions/#operators)

For more information, see:

- [`SELECT` reference page](/self-managed/v2025.01/sql/select/)

- [Query optimization](/self-managed/v2025.01/transform-data/optimization/)

### Views and materialized views

A view represent queries that are saved under a name for reference.

```mzsql
CREATE VIEW my_view_name AS
SELECT ...   ;
```

In Materialize, you can create [indexes](/self-managed/v2025.01/concepts/indexes/#indexes-on-views) on
views. When you to create an index on a view, the underlying query is executed
and the results are stored in memory within the [cluster](/self-managed/v2025.01/concepts/clusters/)
you create the index. As new data arrives, Materialize incrementally updates the
view results.

```mzsql
CREATE INDEX idx_on_my_view ON my_view_name(...) ;
```

You can also create materialized views. A materialized view is a view whose
results are persisted in durable storage. As new data arrives, Materialize
incrementally updates the view results.

```mzsql
CREATE MATERIALIZED VIEW my_mat_view_name AS
SELECT ...  ;
```

You can also create an index on a materialized view to make the results
available in memory within the cluster you create the index.

For more information, see:

- [Views](/self-managed/v2025.01/concepts/views/)
- [Indexes](/self-managed/v2025.01/concepts/indexes/)
- [Indexed views vs materialized
  views](/self-managed/v2025.01/concepts/views/#indexed-views-vs-materialized-views)

### Indexes

In Materialize, [indexes](/self-managed/v2025.01/concepts/indexes/) represent query results stored in
memory within a cluster. By making up-to-date view results available in memory,
indexes can help improve performance within the cluster. Indexes can also help
[optimize query performance](/self-managed/v2025.01/transform-data/optimization/).

For more information, see:

- [Indexes](/self-managed/v2025.01/concepts/indexes)
- [Indexed views vs materialized
  views](/self-managed/v2025.01/concepts/indexes/#indexes-on-views-vs-materialized-views)
- [Indexes: Best practices](/self-managed/v2025.01/concepts/indexes/#best-practices)
