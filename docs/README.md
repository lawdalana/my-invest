# Documentation

Project documentation and guides.

## Directory Structure

```
docs/
├── architecture/
│   ├── system-design.md         # High-level system architecture
│   ├── data-flow.md             # Data flow diagrams
│   ├── api-design.md            # API design decisions
│   └── database-schema.md       # Database schema documentation
├── guides/
│   ├── getting-started.md       # Quick start guide
│   ├── local-development.md     # Local development setup
│   ├── deployment.md            # Deployment guide
│   ├── contributing.md          # Contribution guidelines
│   └── troubleshooting.md       # Common issues and solutions
├── api/
│   ├── authentication.md        # Auth API documentation
│   ├── assets.md                # Assets API documentation
│   ├── watchlists.md            # Watchlists API documentation
│   ├── alerts.md                # Alerts API documentation
│   └── websocket.md             # WebSocket API documentation
├── development/
│   ├── coding-standards.md      # Code style and standards
│   ├── testing-strategy.md      # Testing approach
│   ├── git-workflow.md          # Git workflow and branching
│   └── pull-request-template.md # PR template
└── operations/
    ├── monitoring.md            # Monitoring and alerting
    ├── incident-response.md     # Incident response procedures
    ├── backup-recovery.md       # Backup and recovery procedures
    └── cost-optimization.md     # GCP cost optimization tips
```

## Documentation Standards

- Use Markdown format
- Include code examples where applicable
- Keep diagrams up to date
- Version API documentation with API versions
- Update docs with each feature change

## Key Documents

- **REQUIREMENTS.md**: Complete project requirements (already exists in `/document`)
- **CLAUDE.md**: AI assistant instructions (already exists in root)
- Architecture diagrams should be created using Mermaid or similar tools
- API documentation can be auto-generated from OpenAPI/Swagger specs

## Contributing to Docs

When making changes:
1. Update relevant documentation
2. Review for accuracy
3. Check all links work
4. Include in PR description
