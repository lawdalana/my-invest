# Infrastructure

Infrastructure as Code (IaC) and containerization configurations.

## Directory Structure

```
infrastructure/
├── docker/                          # Docker configurations
│   ├── frontend/
│   │   ├── Dockerfile.dev           # Frontend dev Dockerfile
│   │   └── Dockerfile.prod          # Frontend prod Dockerfile
│   ├── backend/
│   │   ├── Dockerfile.dev           # Backend dev Dockerfile
│   │   └── Dockerfile.prod          # Backend prod Dockerfile
│   └── nginx/
│       └── nginx.conf               # Nginx reverse proxy config
├── terraform/                       # Terraform IaC
│   ├── modules/                     # Reusable Terraform modules
│   │   ├── cloud-run/               # Cloud Run module
│   │   │   ├── main.tf
│   │   │   ├── variables.tf
│   │   │   └── outputs.tf
│   │   ├── cloud-storage/           # Cloud Storage module
│   │   │   ├── main.tf
│   │   │   ├── variables.tf
│   │   │   └── outputs.tf
│   │   ├── vpc/                     # VPC networking module
│   │   │   ├── main.tf
│   │   │   ├── variables.tf
│   │   │   └── outputs.tf
│   │   ├── monitoring/              # Monitoring & logging module
│   │   │   ├── main.tf
│   │   │   ├── variables.tf
│   │   │   └── outputs.tf
│   │   └── secrets/                 # Secret Manager module
│   │       ├── main.tf
│   │       ├── variables.tf
│   │       └── outputs.tf
│   ├── environments/                # Environment-specific configs
│   │   ├── dev/
│   │   │   ├── main.tf              # Dev environment main
│   │   │   ├── variables.tf
│   │   │   ├── terraform.tfvars     # Dev variables
│   │   │   └── backend.tf           # Remote state config
│   │   ├── staging/
│   │   │   ├── main.tf
│   │   │   ├── variables.tf
│   │   │   ├── terraform.tfvars
│   │   │   └── backend.tf
│   │   └── production/
│   │       ├── main.tf
│   │       ├── variables.tf
│   │       ├── terraform.tfvars
│   │       └── backend.tf
│   ├── providers.tf                 # Provider configurations
│   ├── versions.tf                  # Terraform version constraints
│   └── README.md
├── docker-compose.yml               # Production docker-compose
├── docker-compose.dev.yml           # Development docker-compose
└── .dockerignore                    # Docker ignore file
```

## Docker Compose

### Local Development
Run entire stack locally with one command:
```bash
docker-compose -f docker-compose.dev.yml up
```

Services included:
- Frontend (React dev server on port 3000)
- Backend (Rust API on port 8080)
- MongoDB (port 27017)
- Redis (port 6379)

### Production Build
```bash
docker-compose up
```

## Terraform

### Initialize
```bash
cd terraform/environments/dev
terraform init
```

### Plan Changes
```bash
terraform plan
```

### Apply Changes
```bash
terraform apply
```

### Destroy Resources
```bash
terraform destroy
```

## GCP Resources Managed by Terraform

- Cloud Run (Backend API)
- Cloud Storage + CDN (Frontend)
- Artifact Registry (Container images)
- VPC Network (if needed)
- Secret Manager (API keys, credentials)
- Cloud Monitoring (Alerts, dashboards)
- IAM Roles and Service Accounts
- Budget Alerts

## Cost Optimization

All configurations prioritize:
- GCP Free Tier usage
- Auto-scaling to zero
- Minimal resource allocation
- Shared resources where appropriate
