# Hosted Twin Cloud — deploy scaffold

Docker Compose bundle for a **single-tenant** Twin Cloud Control Center with persisted snapshot storage.

Full runbook: [docs/hosted-twin-cloud.md](../../docs/hosted-twin-cloud.md) · Product GA: [docs/hosted-twin-cloud-product.md](../../docs/hosted-twin-cloud-product.md)

## Quick start (Docker Compose)

```bash
cd deploy/twin-cloud-hosted
cp .env.example .env
# Edit SPANDA_API_KEY and optional SPANDA_TENANT_ID
docker compose up --build
```

Edge clients:

```bash
export SPANDA_TWIN_CLOUD_URL=http://localhost:8080
export SPANDA_TWIN_CLOUD_API_KEY="$SPANDA_API_KEY"
spanda twin cloud push examples/showcase/mission_twin/patrol.sd
spanda twin cloud list
```

## Multi-tenant production

Run **one compose stack per tenant** (different `SPANDA_TENANT_ID`, API key, and volume), or operate a shared Control Center with per-key `tenant_id` scoping — see the hosted runbook.

## Kubernetes

```bash
# Build image (from repo root)
docker build -f deploy/twin-cloud-hosted/Dockerfile -t spanda-twin-cloud:latest .

# Raw manifests
kubectl apply -f deploy/twin-cloud-hosted/k8s/namespace.yaml
kubectl apply -f deploy/twin-cloud-hosted/k8s/configmap.yaml
kubectl apply -f deploy/twin-cloud-hosted/k8s/secret.example.yaml  # edit first
kubectl apply -f deploy/twin-cloud-hosted/k8s/pvc.yaml
kubectl apply -f deploy/twin-cloud-hosted/k8s/deployment.yaml
kubectl apply -f deploy/twin-cloud-hosted/k8s/service.yaml
```

## Helm

```bash
helm upgrade --install twin-cloud deploy/twin-cloud-hosted/helm/twin-cloud \
  --set apiKey="$SPANDA_API_KEY" \
  --set tenantId=default
```

## Verify

```bash
./scripts/hosted_twin_cloud_smoke.sh
```
