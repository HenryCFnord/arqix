---
id: class-constraint
label: constraint
iri: arqix:classes/constraint

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

## Constraint

A restriction that frames other requirements. A constraint is not directly testable on its own; it bounds the solution space in which functional and quality requirements are implemented and verified.
