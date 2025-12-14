#!/bin/bash

# Deploy PostgreSQL on microk8s with nginx ingress
# Prerequisites: microk8s with dns, storage, and ingress addons enabled

echo "Deploying PostgreSQL to microk8s..."

# Enable required addons if not already enabled
# echo "Enabling microk8s addons..."
# microk8s enable dns storage ingress

# Apply configurations in order
echo "Creating namespace..."
kubectl apply -f namespace.yaml

echo "Creating persistent storage..."
kubectl apply -f postgres-storage.yaml

echo "Creating configuration and secrets..."
kubectl apply -f postgres-config.yaml

echo "Deploying PostgreSQL..."
kubectl apply -f postgres-deploy.yaml

echo "Setting up ingress..."
kubectl apply -f postgres-ingress.yaml
kubectl apply -f ingress.yaml

# Wait for deployment
echo "Waiting for deployment to be ready..."
kubectl wait --for=condition=available --timeout=300s deployment/postgres -n mcp

echo "Deployment complete!"
echo ""
echo "PostgreSQL is now accessible via:"
echo "  - Internal cluster: postgres-service.mcp.svc.cluster.local:5432"
echo "  - NodePort: <node-ip>:30432"
echo "  - Ingress: postgres.local (add to /etc/hosts pointing to your microk8s node)"
echo ""
echo "Default credentials:"
echo "  - Username: postgres"
echo "  - Password: aiagent"
echo "  - Database: aiagent_db"
echo ""
echo "To connect externally, add the following to your /etc/hosts file:"
echo "  <your-microk8s-node-ip> postgres.local"