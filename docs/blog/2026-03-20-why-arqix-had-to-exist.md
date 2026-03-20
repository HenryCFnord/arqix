# Why arqix had to exist

There is a narrow band between documentation that is too loose to trust and documentation process that is too heavy to keep alive.

That band is where `arqix` belongs.

Many engineering teams already know the individual ingredients of good documentation. They use Markdown. They store files in Git. They write ADRs, requirements notes, runbooks, and architecture sketches. They sometimes add YAML metadata. None of that is unusual.

What is unusual is how often these artifacts still fail to form a coherent working system.

The failure mode is rarely dramatic. It is usually slow and ordinary:

- one large architecture document becomes the default place for everything
- a temporary shortcut survives long enough to become the new structure
- requirements live in one format, decisions in another, and implementation context somewhere else
- references between artifacts become social knowledge instead of explicit links
- when traceability is needed, someone reconstructs it manually

This is not a complaint about Markdown. Plain Markdown is one of the reasons the situation is still recoverable. It is readable without special tooling, diffable in Git, easy to move between systems, and pleasant enough that people actually use it. YAML frontmatter adds just enough room for machine-meaningful metadata without demanding a platform migration.

The problem is that Markdown alone does not prevent structural decay. Given enough time, even clean docs turn into large, nested, weakly connected pages. Teams then write custom parsers, ad hoc scripts, or one-off conventions to recover the structure they wish had been explicit from the start.

`arqix` comes from taking that pattern seriously.

The project asks a modest question: can technical Markdown documents become a real part of the engineering system instead of a parallel narrative about it?

For that to work, documents need to stay human-readable. They also need to become structured enough for machines, stable enough for deterministic processing, and small enough to support review, reuse, and future retrieval workflows.

That does not require a grand platform. It does not require a graph database on day one. It does not require a plugin ecosystem, a web UI, or a universal ontology. Most likely it requires restraint.

So the first version of `arqix` is intentionally small:

- normal Markdown files
- intentional YAML metadata
- smaller units instead of only giant pages
- deterministic assembly into larger outputs
- configuration instead of hardcoded assumptions

The ambition is real, but the implementation should stay pragmatic. The project only matters if it helps with the actual failure modes: drift, monoliths, disconnected artifacts, and the constant friction of trying to recover structure after the fact.

If that works, `arqix` will earn the right to become more capable later.
