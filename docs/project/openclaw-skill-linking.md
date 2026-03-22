## OpenClaw Skill Linking

This repository keeps project-specific agent skills in:

`.agents/skills/`

OpenClaw does not consume this location directly. To make selected project skills available to OpenClaw, they are linked into the OpenClaw global skill directory on the host.

### Rationale

This approach keeps the skill source versioned inside the project repository while still making the skill available to OpenClaw.

Benefits:

- project-local version control
- reuse across tools and agents
- clear separation between source and host-level integration

Trade-offs:

- this is a host-level integration workaround
- symlinks may need to be recreated after OpenClaw updates
- the OpenClaw global skill directory is not the canonical source of truth

### Canonical Source of Truth

Project skills live in:

`~/projects/arqix/.agents/skills/`

The OpenClaw global skill directory only contains symlinks to selected project skills.

Current repository-managed skills include:

- `arqix-repo-readonly`
- `arqix-repo-prepare`

### Example: Link a Project Skill into OpenClaw

```bash
sudo ln -sfn ~/projects/arqix/.agents/skills/arqix-repo-readonly /usr/lib/node_modules/openclaw/skills/arqix-repo-readonly
sudo ln -sfn ~/projects/arqix/.agents/skills/arqix-repo-prepare /usr/lib/node_modules/openclaw/skills/arqix-repo-prepare
```

### Verify the Link

```bash
ls -la /usr/lib/node_modules/openclaw/skills | grep arqix-repo-
```

Expected output should show a symbolic link pointing to the project-local skill directory.

### Operational Rule

When a new project skill should be made available to OpenClaw:

1. Create or update the skill in `.agents/skills/`.
2. Link it into `/usr/lib/node_modules/openclaw/skills/`.
3. Verify the symlink.
4. Restart or reload OpenClaw if required.

### Maintenance Note

After updating OpenClaw, verify that required symlinks still exist and still point to the intended project-local skill directories.
