#!/usr/bin/env python3
"""Consistency checker for arqix requirement documents.

Checks requirement documents under docs/en/architecture/req/ against the
user stories under docs/en/architecture/stories/ and against the authoring
rules in docs/en/processes/requirements-style-guide.md (RFC 2119 subset +
EARS sentence patterns).

This script is a reference implementation for the later arqix (Rust)
checker: stdlib-only, deterministic output ordering, stable rule IDs,
machine-readable JSON output, and stable exit codes.

Exit codes:
    0  no findings
    1  findings (errors or warnings) reported
    2  usage or I/O error

Rule IDs:
    REQ-ID-001   frontmatter id does not match the filename
    REQ-ID-002   id does not match the REQ-XX-YY-ZZ-NN scheme
    REQ-ID-003   iri does not match the id
    REQ-ID-004   slug does not match the filename
    REQ-ID-005   duplicate requirement id
    REQ-ID-006   per-story NN sequence has gaps or duplicates
    REQ-KIND-001 rdf.type is not exactly one requirement subclass
    REQ-LNK-001  story-bound requirement: the owning story (from the
                 ID) must be the first derived-from object; further
                 stories may follow (canonical-owner model)
    REQ-LNK-002  cross-cutting requirement (00-00-00): needs >= 2
                 derived-from objects
    REQ-LNK-003  derived-from references a story that does not exist
    REQ-LNK-004  story has-requirement references a requirement that
                 does not exist
    REQ-LNK-005  asymmetric link: has-requirement without matching
                 derived-from, or vice versa
    REQ-LNK-006  story has no has-requirement link (warning;
                 suppressed by --allow-unlinked-stories)
    REQ-META-001 required meta field missing or empty
    US-ID-001    story id does not match the US-XX-YY-ZZ scheme
    EARS-001     '## Requirement' section missing or without exactly
                 one normative sentence
    EARS-002     sentence does not match any EARS pattern
    EARS-003     forbidden RFC 2119 keyword (MUST, REQUIRED, ...) or
                 lowercase shall/should/may in the normative sentence
    EARS-004     not exactly one keyword from the allowed subset
    EARS-005     keyword does not fit the requirement kind (warning)
    EARS-006     arqix-containing subject outside the allowed forms
                 ('arqix', 'The arqix CLI', or a backticked command;
                 no invented subsystem nouns) (warning)
"""

import argparse
import json
import re
import sys
from pathlib import Path

REQ_DIR = Path("docs/en/architecture/req")
STORY_DIR = Path("docs/en/architecture/stories")

REQ_ID_RE = re.compile(r"^REQ-(\d{2})-(\d{2})-(\d{2})-(\d{2})$")
US_ID_RE = re.compile(r"^US-(\d{2})-(\d{2})-(\d{2})$")
CROSS_CUTTING_DOMAIN = ("00", "00", "00")

KIND_CLASSES = {
    "arqix:classes/functional-requirement": "functional",
    "arqix:classes/quality-requirement": "quality",
    "arqix:classes/constraint": "constraint",
}

ALLOWED_KEYWORDS = ["SHALL NOT", "SHALL", "SHOULD NOT", "SHOULD", "MAY"]
FORBIDDEN_KEYWORDS = [
    "MUST NOT",
    "MUST",
    "REQUIRED",
    "NOT RECOMMENDED",
    "RECOMMENDED",
    "OPTIONAL",
]
# Keyword expected per kind; violations are warnings (EARS-005).
KIND_KEYWORDS = {
    "functional": {"SHALL", "SHALL NOT", "MAY"},
    "quality": {"SHOULD", "SHOULD NOT", "SHALL", "MAY"},
    "constraint": {"SHALL", "SHALL NOT", "MAY"},
}

KEYWORD_ALT = "|".join(ALLOWED_KEYWORDS)
# The EARS core clause: "the <system> <KEYWORD> <response>". A leading
# article is optional so that bare system names ("arqix SHALL ...") work.
CORE = r"(?:[Tt]he\s+)?\S.*?\s(?:%s)\s\S.*" % KEYWORD_ALT

EARS_PATTERNS = [
    ("unwanted-behaviour", re.compile(r"^If\s+.+?,\s+then\s+%s\.$" % CORE)),
    ("event-driven", re.compile(r"^When\s+.+?,\s+%s\.$" % CORE)),
    ("state-driven", re.compile(r"^While\s+.+?,\s+%s\.$" % CORE)),
    ("optional-feature", re.compile(r"^Where\s+.+?,\s+%s\.$" % CORE)),
    (
        "complex",
        re.compile(
            r"^(?:(?:While|When|Where)\s+.+?,\s+|If\s+.+?,\s+then\s+){2,}%s\.$" % CORE
        ),
    ),
    ("ubiquitous", re.compile(r"^%s\.$" % CORE)),
]

REQUIRED_META = ["lifecycle-status", "owner", "created", "updated", "lang"]


class Finding:
    __slots__ = ("path", "rule", "level", "message")

    def __init__(self, path, rule, level, message):
        self.path = str(path)
        self.rule = rule
        self.level = level  # "error" | "warning"
        self.message = message

    def as_dict(self):
        return {
            "path": self.path,
            "rule": self.rule,
            "level": self.level,
            "message": self.message,
        }


def parse_frontmatter(text):
    """Parse the leading '---' block into (fields, body).

    Line-based and deterministic: supports 'key: value' scalars, one
    level of nesting, and '- predicate/object' triple lists, which is
    all the arqix document format uses. Returns (None, text) when no
    frontmatter block is found.
    """
    lines = text.split("\n")
    idx = 0
    while idx < len(lines) and lines[idx].strip() == "":
        idx += 1
    if idx >= len(lines) or lines[idx].strip() != "---":
        return None, text
    end = idx + 1
    while end < len(lines) and lines[end].strip() != "---":
        end += 1
    if end >= len(lines):
        return None, text

    fm_lines = lines[idx + 1 : end]
    body = "\n".join(lines[end + 1 :])

    fields = {"_top": {}, "meta": {}, "rdf_types": [], "triples": []}
    section = None  # current top-level key for nested blocks
    current_triple = None
    for raw in fm_lines:
        if not raw.strip() or raw.lstrip().startswith("#"):
            continue
        indent = len(raw) - len(raw.lstrip(" "))
        line = raw.strip()

        if indent == 0:
            current_triple = None
            m = re.match(r"^([A-Za-z0-9_-]+):\s*(.*)$", line)
            if not m:
                continue
            key, value = m.group(1), m.group(2).strip()
            section = key
            if value not in ("", "{}", "[]"):
                fields["_top"][key] = value
        elif section == "rdf" and line.startswith("- "):
            fields["rdf_types"].append(line[2:].strip())
        elif section == "triples":
            m = re.match(r"^-\s*predicate:\s*(.*)$", line)
            if m:
                current_triple = {"predicate": m.group(1).strip(), "objects": []}
                fields["triples"].append(current_triple)
                continue
            m = re.match(r"^object:\s*(.*)$", line)
            if m and current_triple is not None:
                value = m.group(1).strip()
                if value:
                    current_triple["objects"].append(value)
                continue
            m = re.match(r"^-\s*(arqix:.*)$", line)
            if m and current_triple is not None:
                current_triple["objects"].append(m.group(1).strip())
        elif section == "meta":
            m = re.match(r"^([A-Za-z0-9_-]+):\s*(.*)$", line)
            if m and m.group(2).strip():
                fields["meta"][m.group(1)] = m.group(2).strip()
    return fields, body


def triple_objects(fields, predicate):
    objs = []
    for triple in fields["triples"]:
        if triple["predicate"] == predicate:
            objs.extend(triple["objects"])
    return objs


def normative_sentences(body):
    """Return the non-comment content lines of the '## Requirement' section."""
    lines = body.split("\n")
    collected = []
    in_section = False
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("## ") or stripped.startswith("### "):
            in_section = stripped == "## Requirement"
            continue
        if in_section and stripped and not stripped.startswith("<!--"):
            collected.append(stripped)
    return collected


def classify_sentence(sentence):
    """Return the first matching EARS pattern name, or None."""
    for name, pattern in EARS_PATTERNS:
        if pattern.match(sentence):
            return name
    return None


def keywords_in(sentence):
    found = []
    remaining = sentence
    # Match NOT-forms first so 'SHALL NOT' is not double-counted as 'SHALL'.
    for kw in ALLOWED_KEYWORDS:
        count = len(re.findall(r"\b%s\b" % kw, remaining))
        if count:
            found.extend([kw] * count)
            remaining = re.sub(r"\b%s\b" % kw, " ", remaining)
    return found, remaining


def check_sentence(path, sentence, kind, findings):
    pattern = classify_sentence(sentence)
    if pattern is None:
        findings.append(
            Finding(
                path,
                "EARS-002",
                "error",
                "sentence does not match any EARS pattern: %r" % sentence,
            )
        )

    for kw in FORBIDDEN_KEYWORDS:
        if re.search(r"\b%s\b" % kw, sentence):
            findings.append(
                Finding(
                    path,
                    "EARS-003",
                    "error",
                    "forbidden keyword '%s'; use the SHALL/SHOULD/MAY subset" % kw,
                )
            )
            break

    found, remaining = keywords_in(sentence)
    if re.search(r"\b(shall|should|may)\b", remaining):
        findings.append(
            Finding(
                path,
                "EARS-003",
                "error",
                "lowercase normative keyword in the requirement sentence",
            )
        )
    if len(found) != 1:
        findings.append(
            Finding(
                path,
                "EARS-004",
                "error",
                "expected exactly one normative keyword, found %d (%s)"
                % (len(found), ", ".join(found) or "none"),
            )
        )
    else:
        if kind in KIND_KEYWORDS and found[0] not in KIND_KEYWORDS[kind]:
            findings.append(
                Finding(
                    path,
                    "EARS-005",
                    "warning",
                    "keyword '%s' unusual for kind '%s' (see style guide matrix)"
                    % (found[0], kind),
                )
            )
        subject = core_subject(sentence)
        if subject is not None and "arqix" in subject.lower():
            allowed = (
                subject == "arqix"
                or subject == "The arqix CLI"
                or re.fullmatch(r"`arqix[^`]*`", subject)
            )
            if not allowed:
                findings.append(
                    Finding(
                        path,
                        "EARS-006",
                        "warning",
                        "subject %r is not an allowed arqix subject form "
                        "('arqix', 'The arqix CLI', or a backticked command)"
                        % subject,
                    )
                )
    return pattern


def core_subject(sentence):
    """Return the core-clause subject: text between the last clause comma
    (or sentence start) and the normative keyword, without a leading 'then'."""
    m = re.search(r"\b(?:%s)\b" % KEYWORD_ALT, sentence)
    if not m:
        return None
    prefix = sentence[: m.start()]
    if "," in prefix:
        prefix = prefix.rsplit(",", 1)[1]
    prefix = prefix.strip()
    if prefix.startswith("then "):
        prefix = prefix[5:].strip()
    return prefix


def check_requirement_file(path, text, findings):
    """Structural + sentence checks for one requirement document.

    Returns (req_id, derived_from_objects) for cross-file checks, or None.
    """
    fields, body = parse_frontmatter(text)
    if fields is None:
        findings.append(Finding(path, "REQ-ID-001", "error", "missing frontmatter"))
        return None
    top = fields["_top"]
    req_id = top.get("id", "")
    filename = path.name

    m = REQ_ID_RE.match(req_id)
    if not m:
        findings.append(
            Finding(
                path,
                "REQ-ID-002",
                "error",
                "id %r does not match REQ-XX-YY-ZZ-NN" % req_id,
            )
        )
        return None

    if not filename.startswith(req_id + "-"):
        findings.append(
            Finding(
                path,
                "REQ-ID-001",
                "error",
                "id %s is not the filename prefix" % req_id,
            )
        )
    expected_iri = "arqix:requirements/" + req_id.lower()
    if top.get("iri") != expected_iri:
        findings.append(
            Finding(
                path,
                "REQ-ID-003",
                "error",
                "iri %r, expected %r" % (top.get("iri"), expected_iri),
            )
        )
    slug = top.get("slug", "")
    if filename.startswith(req_id + "-"):
        tail = filename[len(req_id) + 1 : -3]
        if slug != tail:
            findings.append(
                Finding(
                    path,
                    "REQ-ID-004",
                    "error",
                    "slug %r does not match filename tail %r" % (slug, tail),
                )
            )

    kinds = [KIND_CLASSES[t] for t in fields["rdf_types"] if t in KIND_CLASSES]
    kind = kinds[0] if len(kinds) == 1 else None
    if len(kinds) != 1:
        findings.append(
            Finding(
                path,
                "REQ-KIND-001",
                "error",
                "rdf.type must be exactly one requirement subclass, found: %s"
                % (", ".join(fields["rdf_types"]) or "none"),
            )
        )

    for key in REQUIRED_META:
        if key not in fields["meta"]:
            findings.append(
                Finding(path, "REQ-META-001", "error", "meta.%s missing or empty" % key)
            )

    derived = triple_objects(fields, "arqix:properties/derived-from")
    domain = m.group(1), m.group(2), m.group(3)
    if domain == CROSS_CUTTING_DOMAIN:
        # Distinct objects: listing the same story twice is one link.
        if len(set(derived)) < 2:
            findings.append(
                Finding(
                    path,
                    "REQ-LNK-002",
                    "error",
                    "cross-cutting requirement needs >= 2 distinct derived-from "
                    "objects, found %d" % len(set(derived)),
                )
            )
    else:
        owner_iri = "arqix:user-stories/us-%s-%s-%s" % domain
        if not derived or derived[0] != owner_iri:
            findings.append(
                Finding(
                    path,
                    "REQ-LNK-001",
                    "error",
                    "story-bound requirement must list its owning story %r as the "
                    "first derived-from object, found %s" % (owner_iri, derived or "none"),
                )
            )

    sentences = normative_sentences(body)
    if len(sentences) != 1:
        findings.append(
            Finding(
                path,
                "EARS-001",
                "error",
                "'## Requirement' must contain exactly one normative sentence, "
                "found %d" % len(sentences),
            )
        )
    if sentences:
        check_sentence(path, sentences[0], kind, findings)

    return req_id, derived


def load_stories(story_dir, findings):
    """Return {story_iri: (path, has_requirement_objects)}."""
    stories = {}
    # Recursive: a story in a subdirectory must not escape the gate.
    for path in sorted(story_dir.rglob("*.md")):
        fields, _ = parse_frontmatter(path.read_text(encoding="utf-8"))
        if fields is None:
            continue
        story_id = fields["_top"].get("id", "")
        if not US_ID_RE.match(story_id):
            # A malformed id must not silently drop the story from every
            # cross-file check — that would make the whole gate blind to it.
            findings.append(
                Finding(
                    path,
                    "US-ID-001",
                    "error",
                    "story id %r does not match the US-XX-YY-ZZ scheme" % story_id,
                )
            )
            continue
        iri = "arqix:user-stories/" + story_id.lower()
        has_req = triple_objects(fields, "arqix:properties/has-requirement")
        stories[iri] = (path, has_req)
    return stories


def cross_file_checks(requirements, stories, allow_unlinked, findings):
    """requirements: {req_iri: (path, derived_from)}."""
    for req_iri, (path, derived) in sorted(requirements.items()):
        for story_iri in derived:
            if story_iri not in stories:
                findings.append(
                    Finding(
                        path,
                        "REQ-LNK-003",
                        "error",
                        "derived-from references missing story %s" % story_iri,
                    )
                )
            elif req_iri not in stories[story_iri][1]:
                findings.append(
                    Finding(
                        path,
                        "REQ-LNK-005",
                        "error",
                        "derived-from %s has no matching has-requirement in the story"
                        % story_iri,
                    )
                )

    for story_iri, (path, has_req) in sorted(stories.items()):
        for req_iri in has_req:
            if req_iri not in requirements:
                findings.append(
                    Finding(
                        path,
                        "REQ-LNK-004",
                        "error",
                        "has-requirement references missing requirement %s" % req_iri,
                    )
                )
            elif story_iri not in requirements[req_iri][1]:
                findings.append(
                    Finding(
                        path,
                        "REQ-LNK-005",
                        "error",
                        "has-requirement %s has no matching derived-from" % req_iri,
                    )
                )
        if not has_req and not allow_unlinked:
            findings.append(
                Finding(
                    path,
                    "REQ-LNK-006",
                    "warning",
                    "story has no has-requirement link",
                )
            )


def sequence_checks(req_ids, findings, req_dir):
    groups = {}
    seen = {}
    for req_id, path in req_ids:
        if req_id in seen:
            findings.append(
                Finding(
                    path,
                    "REQ-ID-005",
                    "error",
                    "duplicate id %s (also in %s)" % (req_id, seen[req_id].name),
                )
            )
            continue
        seen[req_id] = path
        m = REQ_ID_RE.match(req_id)
        groups.setdefault(m.group(1, 2, 3), []).append(int(m.group(4)))
    for domain, numbers in sorted(groups.items()):
        expected = list(range(1, max(numbers) + 1))
        if sorted(numbers) != expected:
            findings.append(
                Finding(
                    req_dir,
                    "REQ-ID-006",
                    "error",
                    "domain %s-%s-%s: NN sequence %s is not contiguous from 01"
                    % (domain + (sorted(numbers),)),
                )
            )


def run_checks(root, allow_unlinked):
    findings = []
    req_dir = root / REQ_DIR
    story_dir = root / STORY_DIR
    if not story_dir.is_dir():
        print("error: story directory not found: %s" % story_dir, file=sys.stderr)
        return None

    stories = load_stories(story_dir, findings)

    requirements = {}
    req_ids = []
    if req_dir.is_dir():
        # Recursive: a requirement in a subdirectory must not escape the gate.
        for path in sorted(req_dir.rglob("*.md")):
            result = check_requirement_file(
                path, path.read_text(encoding="utf-8"), findings
            )
            if result is None:
                continue
            req_id, derived = result
            req_ids.append((req_id, path))
            requirements["arqix:requirements/" + req_id.lower()] = (path, derived)
        sequence_checks(req_ids, findings, req_dir)

    cross_file_checks(requirements, stories, allow_unlinked, findings)
    findings.sort(key=lambda f: (f.path, f.rule, f.message))
    return findings


def report(findings, as_json):
    errors = sum(1 for f in findings if f.level == "error")
    warnings = len(findings) - errors
    if as_json:
        print(
            json.dumps(
                {
                    "findings": [f.as_dict() for f in findings],
                    "summary": {"errors": errors, "warnings": warnings},
                },
                indent=2,
            )
        )
    else:
        for f in findings:
            print("%s: [%s] %s: %s" % (f.level, f.rule, f.path, f.message))
        print("checked: %d error(s), %d warning(s)" % (errors, warnings))
    return 1 if findings else 0


# --- selftest -----------------------------------------------------------

GOOD_REQ = """---
id: REQ-01-01-01-01
title: Test Requirement
slug: test-requirement
iri: arqix:requirements/req-01-01-01-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: deterministic

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  generated: false
---

## Requirement

When `arqix doc new` is invoked without `--id`, arqix SHALL generate a unique document ID from the configured policy.

### Notes

Selftest fixture.
"""

SELFTEST_SENTENCES = [
    # (sentence, kind, expected pattern, expected rule IDs)
    ("The arqix CLI SHALL NOT write outside the declared scope.", "constraint", "ubiquitous", []),
    ("When a lint finding occurs, arqix SHALL exit non-zero.", "functional", "event-driven", []),
    ("While assembly runs, the logger SHALL append JSONL records.", "functional", "state-driven", []),
    ("If an include target is missing, then the assembler SHALL fail.", "functional", "unwanted-behaviour", []),
    ("Where MCP is enabled, the server MAY expose a read tool.", "functional", "optional-feature", []),
    ("The lint diagnostics SHOULD be actionable without reading source.", "quality", "ubiquitous", []),
    ("The tool MUST reject unknown flags.", "functional", None, ["EARS-002", "EARS-003", "EARS-004"]),
    ("arqix shall do things.", "functional", None, ["EARS-002", "EARS-003", "EARS-004"]),
    ("The tool SHALL do this and SHOULD do that.", "functional", "ubiquitous", ["EARS-004"]),
    ("Renders pages quickly.", "functional", None, ["EARS-002", "EARS-004"]),
    ("The exporter SHOULD emit stable ordering.", "constraint", "ubiquitous", ["EARS-005"]),
    ("The arqix formatter SHALL NOT emit noise.", "constraint", "ubiquitous", ["EARS-006"]),
    ("If a value is current, then `arqix finalise` SHALL NOT rewrite it.", "functional", "unwanted-behaviour", []),
    ("When `arqix fmt` runs, arqix SHALL sort keys deterministically.", "functional", "event-driven", []),
]


def selftest():
    failures = []

    for sentence, kind, expected_pattern, expected_rules in SELFTEST_SENTENCES:
        findings = []
        pattern = check_sentence(Path("selftest"), sentence, kind, findings)
        rules = sorted({f.rule for f in findings})
        if pattern != expected_pattern or rules != sorted(expected_rules):
            failures.append(
                "sentence %r: pattern=%s rules=%s, expected pattern=%s rules=%s"
                % (sentence, pattern, rules, expected_pattern, sorted(expected_rules))
            )

    findings = []
    result = check_requirement_file(
        Path("REQ-01-01-01-01-test-requirement.md"), GOOD_REQ, findings
    )
    if result is None or findings:
        failures.append(
            "good fixture: expected clean parse, got findings: %s"
            % [(f.rule, f.message) for f in findings]
        )
    else:
        req_id, derived = result
        if req_id != "REQ-01-01-01-01" or derived != ["arqix:user-stories/us-01-01-01"]:
            failures.append("good fixture: unexpected parse result")

    bad = GOOD_REQ.replace(
        "arqix:classes/functional-requirement", "arqix:classes/requirement"
    ).replace("iri: arqix:requirements/req-01-01-01-01", "iri: arqix:requirements/wrong")
    findings = []
    check_requirement_file(Path("REQ-01-01-01-01-test-requirement.md"), bad, findings)
    rules = sorted({f.rule for f in findings})
    if rules != ["REQ-ID-003", "REQ-KIND-001"]:
        failures.append("bad fixture: expected [REQ-ID-003, REQ-KIND-001], got %s" % rules)

    multi = GOOD_REQ.replace(
        "object: arqix:user-stories/us-01-01-01",
        "object:\n      - arqix:user-stories/us-01-01-01\n"
        "      - arqix:user-stories/us-02-01-01",
    )
    findings = []
    check_requirement_file(Path("REQ-01-01-01-01-test-requirement.md"), multi, findings)
    if findings:
        failures.append(
            "canonical-owner fixture: expected clean, got %s"
            % sorted({f.rule for f in findings})
        )

    foreign = GOOD_REQ.replace(
        "object: arqix:user-stories/us-01-01-01",
        "object:\n      - arqix:user-stories/us-02-01-01\n"
        "      - arqix:user-stories/us-01-01-01",
    )
    findings = []
    check_requirement_file(Path("REQ-01-01-01-01-test-requirement.md"), foreign, findings)
    rules = sorted({f.rule for f in findings})
    if rules != ["REQ-LNK-001"]:
        failures.append("foreign-owner fixture: expected [REQ-LNK-001], got %s" % rules)

    # The required-meta list must match check_frontmatter.py's — `generated`
    # included — so the two gates can never silently diverge again.
    missing_generated = GOOD_REQ.replace("  generated: false\n", "")
    findings = []
    check_requirement_file(
        Path("REQ-01-01-01-01-test-requirement.md"), missing_generated, findings
    )
    rules = sorted({f.rule for f in findings})
    if rules != ["REQ-META-001"]:
        failures.append("missing-generated fixture: expected [REQ-META-001], got %s" % rules)

    if failures:
        for failure in failures:
            print("selftest FAIL: %s" % failure)
        return 1
    print("selftest OK: %d sentence cases, 4 document fixtures" % len(SELFTEST_SENTENCES))
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--root", default=".", help="repository root (default: .)")
    parser.add_argument("--json", action="store_true", help="machine-readable output")
    parser.add_argument(
        "--allow-unlinked-stories",
        action="store_true",
        help="suppress REQ-LNK-006 warnings while derivation is incomplete",
    )
    parser.add_argument("--selftest", action="store_true", help="run built-in examples")
    args = parser.parse_args(argv)

    if args.selftest:
        return selftest()

    findings = run_checks(Path(args.root), args.allow_unlinked_stories)
    if findings is None:
        return 2
    return report(findings, args.json)


if __name__ == "__main__":
    sys.exit(main())
