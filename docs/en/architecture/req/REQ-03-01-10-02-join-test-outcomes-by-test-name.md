---
id: REQ-03-01-10-02
title: Join Test Outcomes by Test Name
slug: join-test-outcomes-by-test-name
iri: arqix:requirements/req-03-01-10-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: With a JUnit XML report given, coverage rows carry the joined outcome per verifying test; markers absent from the report stay untouched.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix trace coverage` is invoked with a test-results report, arqix SHALL join the report's test outcomes to the verifying markers by test name.

### Notes

Derived from US-03-01-10.
The report format is JUnit XML (the least common denominator of mainstream runners); a test case joins the marker whose attached test name equals its `name` attribute.
A results file refines the picture and never invents evidence: unjoined markers keep their marker-derived status.
