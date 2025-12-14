# PostgreSQL on microk8s

This directory contains Kubernetes configuration files to deploy PostgreSQL with external access via nginx ingress controller on microk8s.

## Prerequisites

1. microk8s installed and running
2. Required addons enabled:
   ```bash
   microk8s enable dns storage ingress
   ```

## Files

- `namespace.yaml` - Creates the database namespace
- `postgres-storage.yaml` - PersistentVolume and PersistentVolumeClaim for data persistence
- `postgres-config.yaml` - ConfigMap with database configuration
- `postgres-secret.yaml` - Secret with database credentials
- `postgres-deploy.yaml` - PostgreSQL deployment and ClusterIP service
- `postgres-ingress.yaml` - Ingress configuration for postgress and NodePort service for external access
- `ingress.yaml` - opens the ports on the nginx ingress controller for none http apps
- `deploy.sh` - Deployment script

## Deployment

1. Make the deployment script executable:
   ```bash
   chmod +x deploy.sh
   ```

2. Run the deployment:
   ```bash
   ./deploy.sh
   ```

## Access Methods

### 1. Internal Cluster Access
```
Host: postgres-service.database.svc.cluster.local
Port: 5432
```

### 2. NodePort Access
```
Host: <microk8s-node-ip>
Port: 30432
```

### 3. Ingress Access
Add to `/etc/hosts`:
```
<microk8s-node-ip> postgres.local
```

Then connect to:
```
Host: postgres.local
Port: 5432 (via ingress)
```

## Default Credentials

- **Username**: postgres
- **Password**: aiagent_password
- **Database**: aiagent_db

## Storage

- **Size**: 5Gi
- **Location**: `/var/lib/microk8s/postgres-data` on the host
- **Storage Class**: microk8s-hostpath

## Security Notes

1. Change the default password in `postgres-secret.yaml` (base64 encode your password)
2. Consider using TLS for production deployments
3. Review and adjust resource limits based on your requirements

## Troubleshooting

Check deployment status:
```bash
microk8s kubectl get pods -n database
microk8s kubectl get svc -n database
microk8s kubectl get ingress -n database
```

View logs:
```bash
microk8s kubectl logs deployment/postgres -n database
```

Test connection:
```bash
# From within cluster
microk8s kubectl run postgres-client --rm -it --image=postgres:15 --restart=Never -- psql -h postgres-service.database.svc.cluster.local -U postgres -d aiagent_db

# External test (install postgresql-client on host)
psql -h <microk8s-node-ip> -p 30432 -U postgres -d aiagent_db
```