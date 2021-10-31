# Admission Webhook Example

## Request

In server

```bash
./target/debug/rust_library_sandbox sandbox actix-web simple-admission-webhook
```

And request

```bash
# http://127.0.0.1:8088/health
curl -X GET -H "Content-Type: application/json"  http://127.0.0.1:8088/health

# http://127.0.0.1:8088/mutate
curl -X POST -H "Content-Type: application/json" -d '{
  "apiVersion": "admission.k8s.io/v1",
  "kind": "AdmissionReview",
  "request": {
    "uid": "705ab4f5-6393-11e8-b7cc-42010a800002",
    "kind": {"group":"autoscaling","version":"v1","kind":"Scale"},
    "resource": {"group":"apps","version":"v1","resource":"deployments"},
    "subResource": "scale",
    "requestKind": {"group":"autoscaling","version":"v1","kind":"Scale"},
    "requestResource": {"group":"apps","version":"v1","resource":"deployments"},
    "requestSubResource": "scale",
    "name": "my-deployment",
    "namespace": "my-namespace",
    "operation": "UPDATE",
    "userInfo": {
      "username": "admin",
      "uid": "014fbff9a07c",
      "groups": ["system:authenticated","my-admin-group"],
      "extra": {
        "some-key":["some-value1", "some-value2"]
      }
    },
    "dryRun": false
  }
}' http://127.0.0.1:8088/mutate
```
