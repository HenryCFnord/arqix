```codex
Work on the current git branch only.

Modify only files under docs/en/architecture/stories.
Change only files that are non-compliant with the rule below.
Do not touch already compliant files.

Task:
Normalize persona references inside the user story sentence so that named personas are replaced by their generic role form.

Rule:
In user story body text, do not use named persona labels such as:
- Mara Maintainer
- Dan Developer
- Quinn QA
- Daria DevOps
- Alex AIOps
- Aria Architect
- Avery Auditor
- Casey Coding Agent

Instead, use only the generic role in the standard user story form:
- as a maintainer
- as a developer
- as a QA engineer
- as a DevOps engineer
- as an AIOps engineer
- as an architect
- as an auditor
- as a coding agent

Normalization requirements:
- Update only the leading user story sentence, e.g. lines of the form:
  As a ..., I want ..., so that ...
- Replace named persona references with the correct generic role
- Preserve the rest of the sentence as much as possible
- Preserve grammar and article usage
- Keep the sentence in English
- Do not rewrite acceptance criteria, notes, titles, filenames, ids, iris, or metadata unless absolutely necessary
- Do not modify files outside docs/en/architecture/stories

Examples:
- 'As a Mara Maintainer, I want ...' -> 'As a maintainer, I want ...'
- 'As Dan Developer, I want ...' -> 'As a developer, I want ...'
- 'As an Aria Architect, I want ...' -> 'As an architect, I want ...'

Before editing, identify the non-compliant files.
Then update only those files.

After applying changes:
- verify that all touched files still use a valid user story sentence
- verify that no touched file still contains a named persona label in the leading user story sentence
- verify that compliant files were left unchanged

Then create a git commit with this message:
Normalize persona references in user story sentences

Use exactly this role mapping:
- Mara Maintainer -> maintainer
- Dan Developer -> developer
- Quinn QA -> QA engineer
- Daria DevOps -> DevOps engineer
- Alex AIOps -> AIOps engineer
- Aria Architect -> architect
- Avery Auditor -> auditor
- Casey Coding Agent -> coding agent
```