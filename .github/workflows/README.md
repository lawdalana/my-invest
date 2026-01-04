# GitHub Actions Workflows

CI/CD pipeline configurations.

## Workflows

```
.github/workflows/
├── ci.yml                    # Continuous Integration (PR checks)
├── deploy-dev.yml            # Auto-deploy to dev environment
├── deploy-staging.yml        # Auto-deploy to staging environment
├── deploy-production.yml     # Manual deploy to production
├── terraform-plan.yml        # Terraform plan on PR
├── terraform-apply.yml       # Terraform apply on merge
├── test-frontend.yml         # Frontend-specific tests
├── test-backend.yml          # Backend-specific tests
├── security-scan.yml         # Security vulnerability scanning
└── dependency-update.yml     # Automated dependency updates
```

## Workflow Descriptions

### ci.yml
Runs on every pull request:
- Lint code (ESLint, Clippy)
- Type checking (TypeScript, Rust)
- Unit tests
- Integration tests
- Build verification

### deploy-dev.yml
Triggered on merge to `develop` branch:
- Build Docker images
- Push to Artifact Registry
- Deploy to dev environment
- Run smoke tests

### deploy-staging.yml
Triggered on merge to `staging` branch:
- Build Docker images
- Push to Artifact Registry
- Deploy to staging environment
- Run smoke tests
- Run E2E tests

### deploy-production.yml
Manual trigger with approval required:
- Build Docker images
- Push to Artifact Registry
- Deploy to production
- Run smoke tests
- Post-deployment monitoring

### terraform-plan.yml
Runs on PR with terraform changes:
- Terraform fmt check
- Terraform validate
- Terraform plan
- Post plan as PR comment

### terraform-apply.yml
Runs on merge to main with terraform changes:
- Terraform apply to appropriate environment
- Update infrastructure

### test-frontend.yml
Comprehensive frontend testing:
- Unit tests (Jest + RTL)
- Integration tests
- E2E tests (Playwright)
- Accessibility tests
- Performance budget checks

### test-backend.yml
Comprehensive backend testing:
- Unit tests
- Integration tests
- API contract tests
- Load tests
- Security tests

### security-scan.yml
Weekly security scanning:
- Dependency vulnerability scan (npm audit, cargo audit)
- Docker image scanning
- SAST (Static Application Security Testing)
- Secret scanning

### dependency-update.yml
Weekly automated dependency updates:
- Create PRs for dependency updates
- Run tests automatically
- Tag for review

## Secrets Required

GitHub Secrets needed:
- `GCP_PROJECT_ID`
- `GCP_SA_KEY` (Service Account JSON)
- `GCP_REGION`
- `MONGODB_URI`
- `REDIS_URL`
- `JWT_SECRET`

## Environment Variables

Environment-specific variables:
- `ENVIRONMENT` (dev/staging/production)
- `FRONTEND_URL`
- `BACKEND_URL`
- `API_VERSION`
