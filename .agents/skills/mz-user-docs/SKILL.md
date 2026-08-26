---
name: mz-user-docs
description: >
  Trigger: editing or reviewing user-facing docs under doc/user/ — concepts
  pages, data/*.yml snippet entries, headless includes, Hugo shortcodes; or
  requests like "incorporate X into the docs", "single-source this content",
  "add a section to the <concepts> page", "review my docs changes". Encodes
  the single-sourcing conventions, entry-authoring rules, accuracy rules, and
  the review checklist for doc/user.
---

# User docs (doc/user) content conventions

The user docs are a Hugo site under `doc/user`. The central convention is
**single-sourcing**: a fact that renders on more than one page must live in
exactly one reusable snippet, never as literal prose on two pages. Duplicated
prose drifts (observed in practice: two copies of the same sentence diverged
by one word).

## Where reusable content lives

- `doc/user/data/*.yml`: named entries, rendered with
  `{{% include-from-yaml data="<file>" name="<entry>" %}}`. The shortcode is
  `doc/user/layouts/shortcodes/include-from-yaml.html` and renders entry
  content via `RenderString`.
- `doc/user/content/headless/*.md`: whole-file snippets, rendered with
  `{{% include-headless "/headless/<name>" %}}`.

File organization patterns, both in use:

- **Topic files** group entries by concept regardless of which pages consume
  them: `hydration-details.yml`, `cluster_details.yml`,
  `index_view_details.yml`.
- **Per-page manifests** hold everything one page includes, even entries
  shared with other pages: `index_details.yml` is the manifest for the
  indexes concept page.
- When related entries split across files, leave **breadcrumb comments** in
  both files saying where the siblings live and what must stay in sync.

## Entry authoring rules

- Entries must stand alone (no dangling "however" or "above/below"
  references), but may be designed to compose in sequence. Record that design
  in a comment.
- Comment each entry with its consumer pages and any constraints. Known
  constraints to record:
  - An entry rendered inside a markdown table cell must be a **single
    paragraph** (multi-paragraph output breaks table rows). Author it with a
    `>-` folded scalar. Use the `{{< >}}` angle form of the include inside
    table cells (precedent: `concepts/_index.md`); use `{{% %}}` elsewhere.
  - No page-root links to any page the entry renders on (self-links). Deep
    anchor links are fine; on the entry's own page they act as same-page
    jumps.
  - Footnotes in an entry render immediately after the snippet, not at the
    page bottom (`RenderString` behavior). Preview before shipping footnotes.
- Prose style: no em-dashes, avoid semicolons, wrap near 80 columns, no
  trailing whitespace.

## Accuracy rules

- Never claim reads are "computationally free". Say results are "already
  computed" or "served from memory". A query can use an index and still
  perform additional computation (joins, aggregations, post-filters).
- Full index scans degrade with data volume. Do not claim they are cheap
  because the data is in memory.
- Scope cost and billing claims to Cloud. Self-managed deployments have no
  per-credit billing.
- Verify object capabilities in the planner instead of inferring. Example:
  indexable object types are enumerated in `plan_create_index` in
  `src/sql/src/plan/statement/ddl.rs`; tables cannot take `IN CLUSTER`
  (neither `CreateTableStatement` nor `CreateTableFromSourceStatement` has a
  cluster field).
- Behavior facts about hydration and snapshotting are single-sourced in
  `hydration-details.yml` and `headless/ingestion/*.md`. Check those before
  writing new claims; the concept pages render them.

## Migrating external or course content

1. Extract facts into a scratch file `doc/user/data/temp_<source>_facts.yml`,
   one named entry per fact, to contrast against the existing pages.
2. Keep the scratch file out of git via the local exclude file, which is
   shared across worktrees:
   `echo 'doc/user/data/temp_<source>_facts.yml' >> "$(git rev-parse --git-common-dir)/info/exclude"`
3. Before writing anything, grep for existing coverage. Include bold and
   wrapped variants in the search (`computationally **free**` escaped a plain
   grep for `computationally free`).
4. Single-source **before** first use: if a fact will land on two pages,
   create the data entry first, then wire both pages to it.
5. As each fact lands, annotate its scratch entry with a
   `# MIGRATED <date>: <where it went>` comment. Record deliberate
   exclusions and the reason.

## Review checklist

Run these before declaring docs changes done:

1. YAML parses:
   `ruby -ryaml -e "YAML.load_file('doc/user/data/<file>.yml')"`.
2. Every linked anchor exists: grep the target page for the heading. A
   heading rename changes its anchor; grep `doc/user` for inbound links to
   the old anchor, ignoring `doc/user/public/` (generated build output, not
   authored). To preserve an old anchor after a rename, add an
   `<a name="old-anchor"></a>` shim above the heading (precedent exists in
   `concepts/clusters.md`).
3. Include references resolve: the data file and entry name, or the headless
   path, actually exist.
4. Blast radius of shared snippets: `grep -rln` the include name; a change
   renders on every consumer page, so check each consumer's context.
5. Shortcode pairs balanced: `{{< note >}}` needs `{{< /note >}}` (a single
   missing brace breaks the page).
6. No trailing whitespace: `grep -n ' $' <files>`.
7. Menu weights are unique within the same parent (weights only compete
   per-parent).
8. Reference-style link definitions at file bottom
   (`` [`SELECT`]: /sql/select/ ``) are an accepted pattern.
