# Install
### Prepare kind with local docker registory
```bash
./scripts/kind-with-registry.sh
```
### Build Container Images
```bash
cargo vendor > .cargo/config.toml

docker build -t admission-webhook:debian -f Dockerfile .
```
Push image to local repository in kind
```bash
docker tag admission-webhook:debian localhost:5001/admission-webhook:debian

docker push localhost:5001/admission-webhook:debian
```
### Install Cert Manager
```bash
kubectl create namespace cert-manager

helm repo add jetstack https://charts.jetstack.io

helm repo update

helm install cert-manager jetstack/cert-manager --namespace cert-manager --version v1.8.0 --set installCRDs=true
```
### Install Admission Webhook
```bash
cd charts/admission-webhook/

kubectl create namespace admission-webhook

helm install -nadmission-webhook admission-webhook .
```