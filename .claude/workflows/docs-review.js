export const meta = {
  name: 'docs-review',
  description: 'Review changed doc/user files against the mz-user-docs conventions',
  whenToUse:
    'After editing user docs (concepts pages, data/*.yml entries, headless includes), to fan out verification of links, anchors, includes, yaml validity, prose rules, and cross-page drift. Optionally pass {files: [...]} as args to review specific files; otherwise reviews files changed vs upstream/main plus uncommitted changes.',
  phases: [
    { title: 'Discover', detail: 'collect changed doc/user files' },
    { title: 'Verify', detail: 'per-file conventions check' },
    { title: 'Cross-page', detail: 'include blast radius, drift, renamed anchors' },
  ],
}

const SKILL = '.agents/skills/mz-user-docs/SKILL.md'

const FILES_SCHEMA = {
  type: 'object',
  required: ['files'],
  properties: { files: { type: 'array', items: { type: 'string' } } },
}

const FINDINGS_SCHEMA = {
  type: 'object',
  required: ['findings'],
  properties: {
    findings: {
      type: 'array',
      items: {
        type: 'object',
        required: ['file', 'severity', 'summary'],
        properties: {
          file: { type: 'string' },
          line: { type: 'number' },
          severity: { enum: ['error', 'warn', 'nit'] },
          summary: { type: 'string' },
        },
      },
    },
  },
}

phase('Discover')

let files = Array.isArray(args?.files) ? args.files : null
if (!files) {
  const found = await agent(
    [
      'In this Materialize repo, list the user-docs files changed on the current branch.',
      'Run: git diff --name-only upstream/main...HEAD -- doc/user',
      'and: git status --porcelain -- doc/user (parse out the paths).',
      'Merge, dedupe, and drop anything under doc/user/public/ (generated) and deleted files.',
      'Return JSON {files: [...]} of repo-relative paths.',
    ].join('\n'),
    { schema: FILES_SCHEMA, label: 'changed-files', phase: 'Discover' }
  )
  files = found?.files ?? []
}

if (files.length === 0) {
  return { files: [], findings: [], note: 'no changed doc/user files found' }
}
log(`reviewing ${files.length} file(s)`)

phase('Verify')

const perFileCheck = (f) =>
  [
    `Review the docs file ${f} against the project conventions.`,
    `First read the conventions in ${SKILL}, then read ${f} and verify:`,
    '- Internal links: each /path/ link maps to a page under doc/user/content and each #anchor matches a heading on the target page (heading text lowercased and hyphenated). Ignore matches in doc/user/public/.',
    '- Includes resolve: {{% include-from-yaml data="X" name="Y" %}} means doc/user/data/X.yml exists and contains an entry named Y; {{% include-headless "/headless/Z" %}} means doc/user/content/headless/Z.md exists.',
    '- If the file is a doc/user/data/*.yml file: it parses (ruby -ryaml), and any entry a consumer renders inside a markdown table cell is a single paragraph.',
    '- Shortcode pairs balanced: every {{< X >}} has a matching close with intact braces.',
    '- Prose rules: no em-dashes, no trailing whitespace, no "computationally free"-style overclaims (including bold variants).',
    'Only report defects you verified by reading the relevant target files; no style opinions beyond the listed rules.',
    'Return JSON {findings: [{file, line, severity: error|warn|nit, summary}]}. Empty array if clean.',
  ].join('\n')

const perFile = await pipeline(files, (f) =>
  agent(perFileCheck(f), {
    schema: FINDINGS_SCHEMA,
    label: `check:${f.split('/').pop()}`,
    phase: 'Verify',
  })
)

phase('Cross-page')

const cross = await agent(
  [
    `These doc/user files changed: ${files.join(', ')}.`,
    `Read the conventions in ${SKILL} first. Then check cross-page concerns:`,
    '1. Blast radius: for each changed file under doc/user/data/ or doc/user/content/headless/, grep -rln doc/user/content for pages that include it, and read each consumer to confirm the changed snippet still fits its context (table cell vs block, no self-links on consumer pages).',
    '2. Drift: for substantive sentences added or edited in changed pages, grep doc/user/content for near-duplicates on other pages that should be single-sourced into a data entry instead.',
    '3. Renamed headings: for headings removed or renamed in the diff (git diff upstream/main...HEAD -- <file>), grep doc/user for inbound links to the old anchors, ignoring doc/user/public/.',
    'Return JSON {findings: [{file, line, severity: error|warn|nit, summary}]}. Empty array if clean.',
  ].join('\n'),
  { schema: FINDINGS_SCHEMA, label: 'cross-page', phase: 'Cross-page' }
)

const findings = [
  ...perFile.filter(Boolean).flatMap((r) => r.findings),
  ...(cross?.findings ?? []),
]
const order = { error: 0, warn: 1, nit: 2 }
findings.sort((a, b) => (order[a.severity] ?? 3) - (order[b.severity] ?? 3))

return { files, findings }
