---
id: class-functional-requirement
label: functional-requirement
iri: arqix:classes/functional-requirement

rdf:
  type:
    - rdfs:Class

rdfs:
  sub-class-of:
    - arqix:classes/requirement

triples: []

properties: {}

external-references:
  - type: specification
    label: "RFC 2119: Key words for use in RFCs to Indicate Requirement Levels"
    uri: https://datatracker.ietf.org/doc/html/rfc2119
  - type: specification
    label: "EARS: Easy Approach to Requirements Syntax"
    uri: https://alistairmavin.com/ears/

owl: {}

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  generated: false
---

## Functional-requirement

A directly testable behavior statement.
A functional requirement describes observable system behavior and is verified by linking tests to it via `arqix:properties/verifies-requirement`.
