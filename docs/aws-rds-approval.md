# AWS RDS Approval

This page explains what it would take for PgPulse to become available as a native extension on Amazon RDS for PostgreSQL.

The short version is simple: publishing PgPulse is not enough. AWS must add the native extension to the RDS PostgreSQL engine image before RDS users can run `CREATE EXTENSION pgpulse;`.

## Why Standard RDS Cannot Install PgPulse Today

The current PgPulse extension is a native PostgreSQL extension written in Rust with `pgrx`.

It produces files like:

```text
pgpulse.so
pgpulse.control
pgpulse--0.2.0.sql
```

On self-managed PostgreSQL, those files are copied into PostgreSQL's extension directories. Then PostgreSQL can load the extension.

Amazon RDS does not provide host access to DB instances. That means users cannot copy `pgpulse.so` into the server filesystem. RDS users can only install extensions that AWS already ships for that PostgreSQL engine version.

On RDS, this command only works if AWS has already included the extension:

```sql
CREATE EXTENSION pgpulse;
```

Adding PgPulse to GitHub, crates.io, PGXN, an apt repository, or a Docker image does not change that.

## What AWS Approval Really Means

For native RDS support, AWS would need to:

- Review PgPulse's purpose and demand.
- Review the extension's security model.
- Build the extension for supported PostgreSQL versions.
- Test it with RDS PostgreSQL.
- Include it in RDS engine releases.
- Document the supported PgPulse versions.
- Own the operational support story for RDS customers.

So this is less like publishing a package and more like getting AWS to productize PgPulse as part of RDS PostgreSQL.

## What PgPulse Must Prove

Before approaching AWS, PgPulse should look stable, safe, and easy to support.

Prepare these items:

- A public repository with a clear license.
- Versioned releases and a changelog.
- Reproducible build instructions.
- PostgreSQL version support matrix.
- Extension upgrade scripts.
- Clear install, upgrade, rollback, and uninstall docs.
- CI for tests, formatting, linting, and security checks.
- A security review of unsafe Rust, FFI, `pgrx`, and `pq-sys` usage.
- A resource profile for memory, CPU, background worker behavior, and crash recovery.
- A clear privilege model.
- A clear explanation of why CloudWatch and SQL-only monitoring are not enough.

## Changes That Would Help Approval

The current native extension is useful, but it should be made more AWS-friendly before asking for RDS support.

Recommended hardening:

- Avoid storing replica passwords in PostgreSQL settings.
- Avoid outbound network connections from inside the PostgreSQL extension.
- Keep RDS-specific replica lag collection in the external exporter.
- Make the background worker optional if possible.
- Keep native extension behavior narrow and predictable.
- Document every required PostgreSQL setting.
- Document failure behavior when the worker crashes or cannot collect data.

The easier PgPulse is to reason about, the easier it is for AWS to evaluate.

## Submission Path

There is no public self-serve portal where a developer uploads a native PostgreSQL extension for RDS.

Use these channels:

- Open an AWS Support case as a feature request for RDS PostgreSQL.
- If available, ask an AWS account team or TAM to route the request to the RDS PostgreSQL product team.
- If available, use AWS Partner Network channels.
- Ask interested users to file their own AWS Support requests.
- Share adoption data from the external RDS exporter.

The request should include:

- PgPulse repository link.
- Release artifacts.
- Documentation link.
- Supported PostgreSQL versions.
- Security notes.
- Test evidence.
- Clear customer use cases.
- Explanation of why native support matters.

The strongest request is not "please package my extension." It is:

```text
RDS PostgreSQL users need better replication health monitoring. PgPulse already has external RDS support and real users. Native extension support would improve fidelity for these specific cases.
```

## PgPulse-lite With `pg_tle`

RDS supports Trusted Language Extensions through `pg_tle`.

This is useful, but it cannot run the current Rust extension. A `pg_tle` extension must be written in trusted languages such as SQL or PL/pgSQL.

PgPulse-lite can provide:

- SQL views over `pg_stat_replication`.
- SQL views over `pg_stat_activity`.
- A PL/pgSQL health function.
- Helper functions that make RDS monitoring queries easier to use.

PgPulse-lite cannot provide:

- Rust native code.
- A PostgreSQL background worker.
- Shared memory snapshots.
- `shared_preload_libraries`.
- Raw `libpq` calls from inside PostgreSQL.

So PgPulse-lite should be treated as an optional helper layer for RDS, not as a replacement for the native extension.

## Recommended Roadmap

1. Build `pgpulse-exporter --mode rds`.
2. Add RDS docs for ECS, EC2, Kubernetes, and VM deployments.
3. Add AWS RDS replica discovery.
4. Add CloudWatch `ReplicaLag` collection.
5. Add PostgreSQL SSL support to the exporter.
6. Add optional PgPulse-lite through `pg_tle`.
7. Harden the native extension for self-managed PostgreSQL.
8. Prepare an AWS RDS extension proposal packet.
9. File AWS Support and account-team requests.
10. Keep collecting user demand and adoption data.

## Proposal Packet

Create a separate proposal packet when PgPulse is ready to approach AWS.

It should contain:

- One-page product summary.
- Customer problem statement.
- Why existing RDS metrics are not enough.
- Architecture overview.
- Security model.
- Privilege model.
- Build and test instructions.
- Supported PostgreSQL versions.
- Known limitations.
- Upgrade and rollback process.
- Operational impact.
- Example user workflow.
- Adoption evidence.

## Practical Recommendation

Do not wait for native RDS approval before serving RDS users.

The fastest useful RDS product is:

```bash
pgpulse-exporter --mode rds
```

That exporter can run outside RDS, connect to RDS over SSL, use CloudWatch for AWS-level lag, and expose Prometheus metrics today.

Native RDS extension support should be a long-term adoption path. PgPulse-lite through `pg_tle` should be a smaller SQL helper path.
