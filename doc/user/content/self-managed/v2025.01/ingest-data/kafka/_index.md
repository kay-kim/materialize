---
title: "Kafka"
description: "Connecting Materialize to a Kafka source."
disable_list: true
menu:
  main:
    parent: 'ingest-data'
    identifier: 'kafka'
    weight: 20
---

Materialize provides native connector for Kafka message broker. To ingest data
from Kafka, you need to

1. Create a connection that specifies access and authentication parameters.
2. Create a source that specifies the format of the data you want to ingest.

## Integration guides

- [Amazon MSK](/self-managed/v2025.01/ingest-data/kafka/amazon-msk/)
- [Confluent Cloud](/self-managed/v2025.01/ingest-data/kafka/confluent-cloud/)
- [Self-hosted Kafka](/self-managed/v2025.01/ingest-data/kafka/kafka-self-hosted/)
- [Warpstream](/self-managed/v2025.01/ingest-data/kafka/warpstream/)

## See also

- [Redpanda Cloud](/self-managed/v2025.01/ingest-data/redpanda/redpanda-cloud/)
- [Redpanda Self-hosted](/self-managed/v2025.01/ingest-data/redpanda/)
