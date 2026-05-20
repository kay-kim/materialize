In Materialize, [indexes](/concepts/indexes) on views compute and, as new data
arrives, incrementally update view results in memory within a
[cluster](/concepts/clusters/) instead of recomputing the results from scratch.

Create an index on `cnt_table1` view. Then, as new change events stream in
through Kafka (as the result of `INSERT`, `UPDATE` and `DELETE` operations in
the upstream database), the index incrementally updates the view results in
memory, so queries from the same cluster can read the already computed results
from memory.

```mzsql
CREATE INDEX idx_cnt_table1_field1 ON cnt_table1(field1);
```

For best practices on when to index a view, see
[Indexes](/concepts/indexes/) and [Views](/concepts/views/).
