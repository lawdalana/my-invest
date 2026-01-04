# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**My-Invest Dashboard** is a Progressive Web Application for tracking and analyzing stock market data across multiple asset types (stocks, cryptocurrencies, ETFs, and bonds). The application provides real-time price monitoring, interactive charts, customizable watchlists, and intelligent price/pattern alerts.

**Tech Stack:**
- Frontend: React + TypeScript + TradingView Lightweight Charts
- Backend: Rust (Actix-web/Axum)
- Databases: MongoDB (persistence) + Redis (cache/real-time)
- Architecture: RESTful API + WebSocket for real-time updates
- Deployment: Progressive Web App (PWA)

See `docs/REQUIREMENTS.md` for complete functional and technical specifications.

## Development Commands

To be added once project setup is complete. Will include:
- Frontend: `npm run dev`, `npm run build`, `npm test`, `npm run lint`
- Backend: `cargo run`, `cargo test`, `cargo build --release`
- Database: Docker Compose commands for MongoDB and Redis

## Development Workflow

### Feature Implementation & Bug Fix Process

**CRITICAL: Every feature implementation or bug fix MUST follow this workflow:**

1. **Git Worktree Setup**
   - Create a new worktree for each feature/bugfix using `git worktree add`
   - Branch naming convention: `feature/<name>` or `fix/<name>`
   - Work in the isolated worktree directory

2. **Implementation Phase** (senior-dev-architect agent)
   - Use the `senior-dev-architect` agent to design and implement the feature/fix
   - Agent will handle architecture decisions and code implementation
   - Ensure all code follows project standards and patterns

3. **Testing Phase** (QA engineer agents)
   - After implementation, AUTOMATICALLY trigger appropriate QA agent:
     - **Frontend changes**: Use `frontend-qa-engineer` agent
     - **Backend changes**: Use `backend-qa-engineer` agent
   - Agent will write and run tests to verify the implementation
   - Fix any issues found during testing

4. **Code Review Phase** (senior-code-reviewer agent)
   - Use `senior-code-reviewer` agent to review the complete implementation
   - Address all feedback and suggestions from the review
   - Ensure code quality, security, and best practices

5. **Pull Request**
   - After all phases pass, create PR from the worktree branch
   - PR description should include:
     - Summary of changes
     - Test plan and results
     - Review findings and resolutions

6. **Cleanup**
   - After PR is merged, remove the worktree: `git worktree remove <path>`

### Git Worktree Commands

```bash
# Create new worktree for feature
git worktree add ../my-invest-feature-name -b feature/feature-name

# List all worktrees
git worktree list

# Remove worktree after PR merged
git worktree remove ../my-invest-feature-name

# Prune stale worktrees
git worktree prune
```

## Documentation Guidelines

**IMPORTANT: All documentation MUST be created in the `/docs` directory.**

When creating any documentation files:
- **Architecture docs** → `/docs/architecture/`
- **API documentation** → `/docs/api/`
- **User guides** → `/docs/guides/`
- **Development docs** → `/docs/development/`
- **Operations docs** → `/docs/operations/`
- **General docs** → `/docs/` (root level)

**Never create documentation files in:**
- Project root (except CLAUDE.md, README.md, STRUCTURE.md)
- Frontend or backend directories (only code-specific READMEs allowed)
- Any other location outside `/docs`

## Architecture

### Project Structure
- `/frontend` - React PWA application
- `/backend` - Rust API server
- `/infrastructure` - Docker & Terraform configurations
- `/shared` - Shared TypeScript type definitions
- `/scripts` - Automation scripts (dev, deploy, ci, terraform)
- `/docs` - **ALL project documentation**
- `/.github/workflows` - CI/CD pipelines

### Key Architectural Decisions

1. **Separation of Concerns**: Frontend and backend are separate projects that communicate via REST API and WebSocket

2. **Caching Strategy**: Redis caches frequently accessed data (current prices, search results) to reduce external API calls and database queries

3. **Real-time Updates**: WebSocket connections for live price updates to subscribed assets, with fallback to polling

4. **PWA Offline Support**: Service worker caches essential assets and data for offline functionality

5. **Alert Processing**: Background job in Rust backend periodically checks alert conditions and triggers notifications

6. **Multi-source Data Aggregation**: Backend normalizes data from multiple market data providers into a consistent format

## Code Standards

### General Principles
- Write clean, maintainable, and well-documented code
- Follow language-specific best practices (TypeScript/React for frontend, Rust for backend)
- Prioritize security: avoid XSS, SQL injection, command injection, and other OWASP Top 10 vulnerabilities
- Keep solutions simple - avoid over-engineering and premature abstractions
- Only add features explicitly requested - no extra "improvements"
- Test all critical functionality

### Documentation Standards
- Use Markdown for all documentation files
- Keep documentation up-to-date with code changes
- Include code examples where helpful
- Use clear, concise language
- Add diagrams for complex architectures (Mermaid format preferred)
