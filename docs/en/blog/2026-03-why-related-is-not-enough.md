---
id: blog-why-related-is-not-enough
title: Why “related” is not enough
slug: why-related-is-not-enough
author: hcf
tags:
  - arqix
  - ontology
  - rdf
  - rdfs
  - owl
  - traceability
  - requirements
  - architecture-as-code
  - ai
status: draft
---

## Why “related” is Not Enough

There is a time in almost every documentation system where everything still looks fine.

There are documents.
There is metadata.
There are links.
There is even a field called `related`.

And for a while, that feels respectable and works just fine.

A persona is related to a user story.
A user story is related to a requirement.
A requirement is related to a test.
A test is related to code.

None of this is false.

And **that** is exactly the problem.

`related` is true in the same way that “somewhere over there” is a direction or “next week” is a good time for an appointment. It is not wrong, but it’s just too weak, too imprecise to carry real meaning. And the moment you want more than a loose pile of references, it starts to fail.

That is the point where I stopped being satisfied with `related`.

### The Problem with Vague Structure

A generic relation solves one problem aartefactes several new ones.

It solves this:

> Two things that are somehow connected and should be connected are now formally connected.

But then it immediately breaks down. Because now you do not know:

- what kind of connection this is,
- whether the connection is even allowed,
- whether something important is missing,
- or whether two links that look similar actually mean entirely different things.

Is a workflow merely linked to a persona, or is it primarily performed by that persona? Does a user story refine a requirement, or does a requirement specify the story? Does an ADR just mention an artefact, or does it constrain its design, guide its implementation, or shape its verification?

A machine cannot do much with that ambiguity.

It can traverse the graph.
It cannot trust the graph.

And once every artefact can be “related” to every other artefact, validation becomes almost meaningless. The graph stays flexible, but only because it has stopped saying anything precise.

That is not semantics. That is surrender.

### What an Ontology Actually is

“Ontology” sounds heavier than it is. In practice, an ontology is just a formal way to define:

- which kinds of things exist,
- which kinds of relationships are allowed, and
- what those relationships mean.

That is all.

For `arqix`, that means being able to say things like:

- a `Workflow` is a planning artefact
- a `UserStory` may belong to a `Workflow`
- a `UserStory` may have one or more `Requirement`s
- a `Requirement` may declare one or more verification methods
- an `ADR` may guide design, implementation, or verification
- supporting documents may reference artefacts and support verification

That is already much more powerful than `related`, because once the relation is explicit, the system can start asking better questions like:

- Is this relation valid?
- Is the target of the correct kind?
- Is a required relation missing?
- Which requirements are verified, and by what evidence?
- Which code artefacts implement which requirements?
- Which tests verify which requirements ?
- Which workflows matter for which users group?
- Which architectural decisions affect which part of the system?

That is the real shift: from vague linkage to explicit meaning.

### RDF, RDFS, and OWL without the Incense

The usual stack behind this is RDF, RDFS, and OWL. Those names tend to trigger either enthusiasm or immediate fatigue. Fair enough. The ideas are simpler than the terminology suggests.

### RDF: Statements

RDF is the base layer.

It says that knowledge can be represented as statements:

- subject
- predicate
- object

Plain English, as simple as it can be. That is the graph. Not “a graph” in the decorative diagram sense, but a graph as a set of meaningful statements. For example:

- `WF-0001 hasStory US-1001`
- `US-1001 hasRequirement REQ-0001`
- `IT-0001 verifiesRequirement REQ-0001`

That is already enough to move from “these things are somehow connected” to “this thing stands in this exact relation to that thing.”

### RDFS: Schema

RDFS adds schema.

Now you can say:

- `Workflow` is a class
- `UserStory` is a class
- `hasStory` has domain `Workflow`
- `hasStory` has range `UserStory`

Now the graph has structure, not just edges. It becomes possible to say not only that a relation exists, but whether it makes sense.

### OWL: Stronger Semantics

OWL adds stronger semantics.

That includes things like:

- inverse relations
- disjoint classes
- functional properties
- simple restrictions

For example:

- `hasStory` is the inverse of `isPartOfWorkflow`
- `translationOf` has an inverse `hasTranslation`
- a workflow may have one primary persona
- one artefact may supersede another

For `arqix`, this does not mean building a full semantic-web cathedral with liturgical RDF chanting. It means using a small, useful subset so that the model becomes coherent, checkable, and worth the effort.

### Why This is Better than a Simple Mapping

A simple mapping approach is seductive because it looks tidy. You can group things by category:

- related personas
- related requirements
- related stories

That is better than nothing, but it is still not enough.


**Because grouping is not meaning.**

A grouped list tells you what is nearby. It does not tell you what the connection actually is. That matters more than it seems, because typed relations are what make the graph useful for:

- validation,
- traversal,
- derived views,
- traceability,
-  automation, and
-  especially AI-driven retrieval.

If all the system knows is that several nodes are “related,” then all it can really recover is proximity.

Not intent.
Not role.
Not structure.
Not obligation.
Not evidence.

A typed graph is not merely more formal. It is more honest.

### Planning Stops Being Hand-wavy

The planning model becomes much clearer once the relationships are explicit.

A workflow is not just a note with a nice title. It is an end-to-end process with a goal, an entry point, and an end state.

A user story is not just floating nearby. It may be part of a workflow. It may be decomposed into one or more requirements. It may use defined domain terms.

A requirement is not just a stronger user story. It is a more precise, verifiable statement. It can define verification methods. It can be supported by documentation, reports, code, and tests.

An ADR is not just a linked text file. It can guide design. It can guide implementation. It can guide verification.

A glossary is not optional decoration. It gives the domain language its edges. Without that, the rest of the system drifts into local dialect and silent assumption.

Once these links are typed, the planning layer stops being a bag of markdown files and starts becoming a navigable, checkable model.

That is a very different thing.

### Verification Stops Being Fuzzy Too

Verification is one of the places where weak structure causes the most damage: It is not enough to say that a requirement is somehow connected to a test or a document. A requirement should be able to declare *how it is meant to be verified*. That can be modeled through controlled verification methods such as:

- Inspection
- Analysis
- Demonstration
- Test

Then the requirement becomes more than a wish. It starts carrying its own verification logic. And then the evidence artefacts can say not only which requirement they support, but also which verification method they realise.

That matters.

Because now the system can distinguish:

- a document that supports inspection,
- a report that supports demonstration,
- code that supports analysis, and
- a test artefact that supports test-based verification.

That is not bureaucracy. That is clarity.

### Code Annotations Are where it Gets Interesting

The same logic applies once the graph reaches code. If the ontology is loadable and not hardcoded, then code annotations cannot just be random tags with good intentions. They have to be interpreted in light of the ontology the project has loaded. That means the process becomes something like this:

1. Load the ontology
2. Parse the Markdown artefacts
3. Parse the code annotations
4. Normalize everything into one internal graph model
5. Validate and derive outputs

The code annotations do not need the full weight of YAML. That would be absurd inside real code, but they can still express graph statements in a compact form. *That is the interesting part.* Code stops being just executable text. It becomes semantically linkable.

Now a machine can see, in explicit terms:

- which requirement a piece of code implements,
- which story it realises, or
- which tests verify it.

That is where traceability stops being paperwork and starts becoming an engineering property.

### Machine-readable is Not the Same as Machine-understandable

This distinction matters.

A lot of systems are machine-readable in the weakest possible sense.

You can parse the files.
You can extract the YAML.
You can scan the comments.
You can store the JSON.

Wonderful.

That still does not mean the system carries enough meaning for a machine to do useful work with it. A graph of typed nodes and typed relations is still not “understanding” in the human sense. But it is much closer to interpretable structure than a pile of vaguely connected fields.

**That is exactly why this matters for AI.**

If you want retrieval, GraphRAG, agent workflows, or AI-assisted implementation to work well, then generic edges are not enough. The graph has to preserve the semantics that the AI layer is supposed to use. Otherwise the machine only learns that many things are loosely adjacent.

That is not knowledge.
That is neighborhood.

### Is This Overengineered?

Probably, by ordinary standards.

But that question is too shallow to be useful. The better question is:

> Does this reduce more complexity than it introduces?

If the result were just more files, more syntax, more schemas, more ceremony, and more diagrams for their own sake, then yes, it would be a ridiculous machine for producing organized misery.

**But that is not the point.**

The point is to absorb the complexity once, in the tooling and the model, so that the repeated manual work gets cheaper:

- less custom parsing
- less semantic drift
- less fake traceability
- less disconnected documentation
- less guessing in AI-assisted implementation
- more derived artefacts from one coherent source base

That is the bet.

And I think it is a good one.

### This is Also a Quality Measure for Vibe Coding

This is where the whole thing becomes useful in a slightly ominous way.

Imagine an AI coding workflow where the assistant is not allowed to improvise in semantic darkness. Imagine something like Codex being required, through a skill or workflow, to call `arqix` and operate in a project where:

- requirements exist as typed artefacts
- stories are linked to requirements
- workflows are linked to personas
- ADRs guide implementation choices
- verification methods are explicit
- tests explain what they verify
- code annotations connect implementation back to planning artefacts
- human-readable documentation is generated alongside machine-usable graph data

That does not make sloppy work impossible.

But it raises the floor.

It means fast iteration still has to leave behind an interpretable trail. And that might be one of the most practical quality controls for AI-assisted development: not banning speed, not pretending everything must become ceremony, but forcing the system to remain explainable.

### What arqix is Trying to Do

At that point, `arqix` is no longer just a Markdown tool but it becomes a compiler. It goes from

- human-readable documentation,
- lightweight code annotations, and
- explicit ontology definitions

into

- a coherent engineering knowledge graph,
- traceability views,
- verification support,
- publishable artefacts, and
- and machine-usable context for AI systems.

That is why `related` is not enough. `related` is what you use when you know things are connected. An ontology is what you use when you want those connections to carry meaning. And once they carry meaning, a tool can finally start doing real work.
