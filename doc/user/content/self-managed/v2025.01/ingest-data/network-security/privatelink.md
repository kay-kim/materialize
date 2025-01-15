---
title: "AWS PrivateLink connections"
description: "How to connect Materialize to a Kafka broker, or a PostgreSQL database using an AWS PrivateLink connection"
aliases:
  - /self-managed/v2025.01/ops/network-security/privatelink/
  - /self-managed/v2025.01/connect-sources/privatelink/
menu:
  main:
    parent: "network-security"
    name: "AWS PrivateLink connections"
---

[//]: # "TODO(morsapaes) Add shortcode with instructions for AWS RDS MySQL"

Materialize can connect to a Kafka broker, a Confluent Schema Registry server, a
PostgreSQL database, or a MySQL database through an [AWS PrivateLink](https://aws.amazon.com/privatelink/)
service.

In this guide, we'll cover how to create `AWS PRIVATELINK` connections
and retrieve the AWS principal needed to configure the AWS PrivateLink service.

## Create an AWS PrivateLink connection

{{< tabs tabID="1" >}}
{{< tab "Kafka on AWS">}}

{{< note >}}
Materialize provides Terraform modules for both [MSK cluster](https://github.com/MaterializeInc/terraform-aws-msk-privatelink) and [self-managed Kafka clusters](https://github.com/MaterializeInc/terraform-aws-kafka-privatelink) which can be used to create the target groups for each Kafka broker (step 1), the network load balancer (step 2),
the TCP listeners (step 3) and the VPC endpoint service (step 5).
{{< /note >}}

{{% network-security/privatelink-kafka %}}

{{< /tab >}}

{{< tab "AWS RDS">}}

{{% network-security/privatelink-postgres %}}

{{< /tab >}}

{{< /tabs >}}

## Related pages

- [`CREATE SECRET`](/self-managed/v2025.01/sql/create-secret)
- [`CREATE CONNECTION`](/self-managed/v2025.01/sql/create-connection)
- [`CREATE SOURCE`: Kafka](/self-managed/v2025.01/sql/create-source/kafka)
- Integration guides: [Self-hosted
  PostgreSQL](/self-managed/v2025.01/ingest-data/postgres/self-hosted/), [Amazon RDS for
  PostgreSQL](/self-managed/v2025.01/ingest-data/postgres/amazon-rds/), [Self-hosted
  Kafka](/self-managed/v2025.01/ingest-data/kafka/kafka-self-hosted), [Amazon
  MSK](/self-managed/v2025.01/ingest-data/kafka/amazon-msk), [Redpanda
  Cloud](/self-managed/v2025.01/ingest-data/redpanda/redpanda-cloud/)
