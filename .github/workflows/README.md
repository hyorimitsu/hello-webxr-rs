How to set up OIDC in Google Cloud
---

```shell
# initialize gcloud
gcloud init

# create a service account
gcloud iam service-accounts create "gha-oidc" --project "hello-webxr-rs"

# enable Service Account Credential API
gcloud services enable iamcredentials.googleapis.com --project "hello-webxr-rs"

# create a new workload identity pool
gcloud iam workload-identity-pools create "gha-oidc-idp" \
    --project="hello-webxr-rs" \
    --location="global" \
    --display-name="GitHub Actions OIDC IdP"

# describe a workload identity pool
gcloud iam workload-identity-pools describe "gha-oidc-idp" \
    --project="hello-webxr-rs" \
    --location="global" \
    --format="value(name)"

# create a new OIDC workload identity pool provider
gcloud iam workload-identity-pools providers create-oidc "gha-oidc-provider" \
    --project="hello-webxr-rs" \
    --location="global" \
    --workload-identity-pool="gha-oidc-idp" \
    --display-name="GitHub Actions OIDC Provider" \
    --attribute-mapping="google.subject=assertion.sub,attribute.actor=assertion.actor,attribute.repository=assertion.repository" \
    --issuer-uri="https://token.actions.githubusercontent.com"

# add an IAM policy binding to an IAM service account
gcloud iam service-accounts add-iam-policy-binding "gha-oidc@hello-webxr-rs.iam.gserviceaccount.com" \
    --project="hello-webxr-rs" \
    --role="roles/iam.workloadIdentityUser" \
    --member="principalSet://iam.googleapis.com/projects/11725892241/locations/global/workloadIdentityPools/gha-oidc-idp/attribute.repository/hyorimitsu/hello-webxr-rs"

# add IAM policy binding for a project
gcloud projects add-iam-policy-binding hello-webxr-rs \
    --project="hello-webxr-rs" \
    --role="roles/firebasehosting.admin" \
    --member="serviceAccount:gha-oidc@hello-webxr-rs.iam.gserviceaccount.com"

# describe a workload identity pool provider
gcloud iam workload-identity-pools providers describe "gha-oidc-provider" \
    --project="hello-webxr-rs" \
    --location="global" \
    --workload-identity-pool="gha-oidc-idp" \
    --format="value(name)"
```
