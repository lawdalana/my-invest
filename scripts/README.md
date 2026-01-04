# Scripts

Utility scripts for development, deployment, and maintenance.

## Directory Structure

```
scripts/
├── dev/
│   ├── setup-local.sh           # Initial local environment setup
│   ├── seed-db.sh               # Seed database with sample data
│   └── reset-db.sh              # Reset local databases
├── deploy/
│   ├── deploy-frontend.sh       # Deploy frontend to GCP
│   ├── deploy-backend.sh        # Deploy backend to GCP
│   └── deploy-full.sh           # Deploy entire stack
├── ci/
│   ├── run-tests.sh             # Run all tests
│   ├── build-images.sh          # Build Docker images
│   └── push-images.sh           # Push to Artifact Registry
├── terraform/
│   ├── tf-init.sh               # Initialize Terraform
│   ├── tf-plan.sh               # Run Terraform plan
│   ├── tf-apply.sh              # Apply Terraform changes
│   └── tf-destroy.sh            # Destroy Terraform resources
└── utils/
    ├── generate-types.sh        # Generate shared types
    ├── check-dependencies.sh    # Check for outdated dependencies
    └── clean.sh                 # Clean build artifacts
```

## Usage

All scripts should be run from the repository root:

```bash
# Setup local development environment
./scripts/dev/setup-local.sh

# Seed database with sample data
./scripts/dev/seed-db.sh

# Deploy to production
./scripts/deploy/deploy-full.sh production

# Run all tests
./scripts/ci/run-tests.sh
```

## Naming Convention

- `setup-*.sh` - Initial setup scripts
- `deploy-*.sh` - Deployment scripts
- `run-*.sh` - Execution scripts
- `check-*.sh` - Validation scripts
- `*-db.sh` - Database-related scripts
