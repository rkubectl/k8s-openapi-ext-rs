# k8s-openapi-ext
Extensions for k8s-openapi crate. Collection of fluent builder traits for Kubernetes objects.
See [crates.io](https://crates.io/crates/k8s-openapi-ext) for more details.

## Versioning
This project follows versioning of [k8s-openapi](https://crates.io/crates/k8s-openapi).
For example, if you are using `k8s-openapi` version `0.27`, you should use `k8s-openapi-ext` version `0.27` as well.

## Features
You need to enable corresponding `k8s-openapi` features according to the Kubernetes version you are targeting.
For example in your `Cargo.toml`:

```toml
[dependencies]
k8s-openapi = { version = "0.27", features = ["latest"] }
```
