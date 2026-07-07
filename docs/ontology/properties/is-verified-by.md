---
id: property-is-verified-by
label: is-verified-by
iri: arqix:properties/is-verified-by

rdf:
  type:
    - rdf:Property

rdfs:
  domain:
    - arqix:classes/requirement
  range:
    - arqix:classes/verification-artefact

owl:
  inverse-of: arqix:properties/verifies-requirement

triples: []

properties: {}

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-03
  lang: en
  generated: false
---

## Is-verified-by

Relates a requirement to a verification artefact that verifies it.
Inverse of `verifies-requirement`.
