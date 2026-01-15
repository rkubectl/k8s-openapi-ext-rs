# k8s-openapi-ext

Extensions for k8s-openapi crate. Collection of fluent builder traits for Kubernetes objects.
See [crates.io](https://crates.io/crates/k8s-openapi-ext) for more details.

## Versioning

This project follows versioning of [k8s-openapi](https://crates.io/crates/k8s-openapi).
For example, if you are using `k8s-openapi` version `0.27`, you should use `k8s-openapi-ext` version `0.27` as well.

## k8s-openapi features (K8s version selection)

You need to enable corresponding `k8s-openapi` features according to the Kubernetes version you are targeting.
For example in your `Cargo.toml`:

```toml
[dependencies]
k8s-openapi = { version = "0.27", features = ["latest"] }
k8s-openapi-ext = "0.27"
```

**Note**: All common Kubernetes API modules (e.g., `corev1`, `appsv1`, `metav1`) are conveniently re-exported by `k8s-openapi-ext`. The original `k8s-openapi` crate is also re-exported as `openapi` when you need direct access to its functionality.

## API Overview

This crate provides fluent builder traits for creating and working with Kubernetes resources:

- **Builder Traits (`*Ext`)**: Over 30 traits for fluent resource construction (e.g., `PodExt`, `ServiceExt`, `DeploymentExt`)
- **Getter Traits (`*GetExt`)**: Convenient accessors for extracting data from resources (e.g., `PodGetExt`, `ContainerGetExt`)
- **`ResourceBuilder`**: Common functionality for metadata, labels, annotations, and recommended Kubernetes labels
- **Convenient Re-exports**: All common Kubernetes API modules are re-exported (e.g., `corev1`, `appsv1`, `metav1`) for easier imports
- **Well-known Labels**: Constants for standard Kubernetes labels via the `label` module (e.g., `label::APP_NAME`, `label::APP_VERSION`)

The original `k8s-openapi` crate is re-exported as `openapi` for direct access when needed.

All builder traits support method chaining and follow consistent naming patterns. Getter traits provide ergonomic access to nested fields that would otherwise require verbose option chaining.

## Usage Examples

### Basic Pod Creation

```rust
use k8s_openapi_ext::{corev1, ContainerExt, PodExt, PodSpecExt};

// Create a simple pod
let pod = corev1::Pod::new("my-app")
    .namespace("default")
    .labels([("app", "my-app"), ("version", "1.0")])
    .spec(
        corev1::PodSpec::container(
            corev1::Container::new("app")
                .image("nginx:1.21")
                .port(80)
        )
        .restart_policy("Always")
    );
```

### Advanced Pod with Method Chaining

```rust
use k8s_openapi_ext::{corev1, ContainerExt, PodExt, PodSpecExt};

let pod = corev1::Pod::new("complex-app")
    .namespace("production")
    .app_name("my-application")
    .app_version("2.1.0")
    .spec(
        corev1::PodSpec::container(
            corev1::Container::new("app")
                .image("myapp:2.1.0")
                .port(8080)
                .env("DATABASE_URL", "postgres://...")
        )
        .hostname("my-pod")
        .host_network(false)
        .dns_policy("ClusterFirst")
        .restart_policy("Always")
        .priority(100)
        .service_account_name("my-service-account")
        .image_pull_secret("my-registry-secret")
        .node_selector([("zone", "us-west-2a"), ("node-type", "compute")])
        .termination_grace_period_seconds(30)
    );
```

### Service Creation

```rust
use k8s_openapi_ext::{corev1, ServiceExt, ServicePortExt};

// ClusterIP service
let service = corev1::Service::cluster_ip(
    "my-service",
    [corev1::ServicePort::new(80).target_port(8080)]
)
.namespace("default")
.selector([("app", "my-app")])
.app_name("my-application");

// LoadBalancer service
let lb_service = corev1::Service::load_balancer("public-service")
    .namespace("default")
    .labels([("tier", "frontend")])
    .selector([("app", "frontend")]);
```

### Working with Existing Resources (GetExt traits)

```rust
use k8s_openapi_ext::{corev1, PodGetExt, ContainerGetExt};

fn analyze_pod(pod: &corev1::Pod) {
    // Use PodGetExt for convenient access
    if let Some(phase) = pod.phase() {
        println!("Pod phase: {}", phase);
    }

    if pod.is_running() {
        println!("Pod is running!");
    }

    // Access containers easily
    if let Some(containers) = pod.containers() {
        for container in containers {
            println!("Container: {}", container.name());
            if let Some(image) = container.image() {
                println!("  Image: {}", image);
            }
        }
    }

    // Check node selector
    if let Some(node_selector) = pod.node_selector() {
        println!("Node selector: {:?}", node_selector);
    }
}
```

### Deployment Creation

```rust
use k8s_openapi_ext::{appsv1, corev1, DeploymentExt, PodTemplateSpecExt, ContainerExt};

let deployment = appsv1::Deployment::new("web-app")
    .namespace("production")
    .app_name("web-application")
    .replicas(3)
    .match_labels([("app", "web-app")])
    .template(
        corev1::PodTemplateSpec::new()
            .app_name("web-application")
            .app_version("1.0.0")
            .spec(
                corev1::PodSpec::container(
                    corev1::Container::new("web")
                        .image("nginx:1.21")
                        .port(80)
                )
            )
    );
```

### Using Well-known Labels

```rust
use k8s_openapi_ext::{corev1, label, PodExt, ContainerExt, PodSpecExt};

// Using standard Kubernetes labels with constants
let pod = corev1::Pod::new("my-app")
    .namespace("default")
    .label(label::APP_NAME, "my-application")
    .label(label::APP_VERSION, "1.2.3")
    .label(label::APP_COMPONENT, "web-server")
    .label(label::APP_MANAGED_BY, "helm")
    .spec(
        corev1::PodSpec::container(
            corev1::Container::new("web")
                .image("nginx:1.21")
                .port(80)
        )
    );

// Or use the convenience methods (which use the same labels internally)
let pod_with_convenience = corev1::Pod::new("my-app")
    .namespace("default")
    .app_name("my-application")       // Uses label::APP_NAME
    .app_version("1.2.3")             // Uses label::APP_VERSION
    .app_component("web-server")      // Uses label::APP_COMPONENT
    .app_managed_by("helm")           // Uses label::APP_MANAGED_BY
    .spec(
        corev1::PodSpec::container(
            corev1::Container::new("web")
                .image("nginx:1.21")
                .port(80)
        )
    );
```

## Features

### `time`

The `time` feature enables additional time conversion methods in the `TimeExt` trait for working with Kubernetes timestamps using the popular [`time`](https://crates.io/crates/time) crate.

When enabled, you get additional methods for converting between `metav1::Time` and `time::UtcDateTime`:

```toml
[dependencies]
k8s-openapi = { version = "0.27", features = ["latest"] }
k8s-openapi-ext = { version = "0.27", features = ["time"] }
time = "0.3"
```

```rust
use k8s_openapi_ext::{metav1, TimeExt};

// Convert from time::UtcDateTime to metav1::Time
let time_utc = time::UtcDateTime::now_utc();
let k8s_time = metav1::Time::try_from_utc_date_time(time_utc)?;

// Convert from metav1::Time to time::UtcDateTime
let k8s_time = metav1::Time::now();
let time_utc = k8s_time.to_utc_date_time();
```

Without the `time` feature, you can still work with Kubernetes timestamps using `std::time::SystemTime` and the `jiff` crate (which is used internally).
