use super::*;

pub trait PodSpecExt {
    /// Build `corev1::PodSpec` with this container
    ///
    fn container(container: corev1::Container) -> Self;

    /// Build `corev1::PodSpec` with these containers
    ///
    fn containers(containers: impl IntoIterator<Item = corev1::Container>) -> Self;

    /// Set `active_deadline_seconds`
    ///
    fn active_deadline_seconds(self, seconds: i64) -> Self;

    /// Set affinity
    ///
    fn affinity(self, affinity: impl Into<Option<corev1::Affinity>>) -> Self;

    /// Set `automount_service_account_token`
    ///
    fn automount_service_account_token(self, yes: bool) -> Self;

    /// Set `dns_policy`
    ///
    fn dns_policy(self, policy: impl ToString) -> Self;

    /// Set `enable_service_links`
    ///
    fn enable_service_links(self, yes: bool) -> Self;

    /// Set `hostname`
    ///
    fn hostname(self, hostname: impl ToString) -> Self;

    /// Set `host_ipc`
    ///
    fn host_ipc(self, yes: bool) -> Self;

    /// Set `host_network`
    ///
    fn host_network(self, yes: bool) -> Self;

    /// Set `host_pid`
    ///
    fn host_pid(self, yes: bool) -> Self;

    /// Add image pull secret
    ///
    fn image_pull_secret(self, name: impl ToString) -> Self;

    /// Set `node_name`
    ///
    fn node_name(self, node_name: impl ToString) -> Self;

    /// Add node selector
    ///
    fn node_selector(
        self,
        node_selector: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    /// Set `preemption_policy`
    ///
    fn preemption_policy(self, policy: impl ToString) -> Self;

    /// Set `priority`
    ///
    fn priority(self, priority: i32) -> Self;

    /// Set `priority_class_name`
    ///
    fn priority_class_name(self, class_name: impl ToString) -> Self;

    /// Set `restart_policy`
    ///
    fn restart_policy(self, policy: impl ToString) -> Self;

    /// Set `runtime_class_name`
    ///
    fn runtime_class_name(self, class_name: impl ToString) -> Self;

    /// Set `scheduler_name`
    ///
    fn scheduler_name(self, scheduler: impl ToString) -> Self;

    /// Set service account name
    ///
    fn service_account_name(self, name: impl ToString) -> Self;

    /// Set `service_account` (deprecated)
    ///
    /// **Deprecated:** Use [`service_account_name`] instead.
    #[deprecated(note = "Use service_account_name instead")]
    fn service_account(self, name: impl ToString) -> Self;

    /// Set `set_hostname_as_fqdn`
    ///
    fn set_hostname_as_fqdn(self, yes: bool) -> Self;

    /// Set `share_process_namespace`
    ///
    fn share_process_namespace(self, yes: bool) -> Self;

    /// Set `subdomain`
    ///
    fn subdomain(self, subdomain: impl ToString) -> Self;

    /// Set `termination_grace_period_seconds`
    ///
    fn termination_grace_period_seconds(self, seconds: i64) -> Self;

    /// Add tolerations
    ///
    fn tolerations(self, tolerations: impl IntoIterator<Item = corev1::Toleration>) -> Self;

    /// Add `volumes`
    ///
    fn volumes(self, volumes: impl IntoIterator<Item = corev1::Volume>) -> Self;
}

impl PodSpecExt for corev1::PodSpec {
    fn container(container: corev1::Container) -> Self {
        let containers = vec![container];
        Self {
            containers,
            // active_deadline_seconds: todo!(),
            // affinity: todo!(),
            // automount_service_account_token: todo!(),
            // dns_config: todo!(),
            // dns_policy: todo!(),
            // enable_service_links: todo!(),
            // ephemeral_containers: todo!(),
            // host_aliases: todo!(),
            // host_ipc: todo!(),
            // host_network: todo!(),
            // host_pid: todo!(),
            // hostname: todo!(),
            // image_pull_secrets: todo!(),
            // init_containers: todo!(),
            // node_name: todo!(),
            // node_selector: todo!(),
            // overhead: todo!(),
            // preemption_policy: todo!(),
            // priority: todo!(),
            // priority_class_name: todo!(),
            // readiness_gates: todo!(),
            // restart_policy: todo!(),
            // runtime_class_name: todo!(),
            // scheduler_name: todo!(),
            // security_context: todo!(),
            // service_account: todo!(),
            // service_account_name: todo!(),
            // set_hostname_as_fqdn: todo!(),
            // share_process_namespace: todo!(),
            // subdomain: todo!(),
            // termination_grace_period_seconds: todo!(),
            // tolerations: todo!(),
            // topology_spread_constraints: todo!(),
            // volumes: todo!(),
            ..default()
        }
    }

    fn containers(containers: impl IntoIterator<Item = corev1::Container>) -> Self {
        let containers = Vec::from_iter(containers);
        Self {
            containers,
            ..default()
        }
    }

    fn active_deadline_seconds(self, seconds: i64) -> Self {
        Self {
            active_deadline_seconds: Some(seconds),
            ..self
        }
    }

    fn affinity(self, affinity: impl Into<Option<corev1::Affinity>>) -> Self {
        let affinity = affinity.into();
        Self { affinity, ..self }
    }

    fn automount_service_account_token(self, yes: bool) -> Self {
        Self {
            automount_service_account_token: Some(yes),
            ..self
        }
    }

    fn dns_policy(self, policy: impl ToString) -> Self {
        let dns_policy = Some(policy.to_string());
        Self { dns_policy, ..self }
    }

    fn enable_service_links(self, yes: bool) -> Self {
        Self {
            enable_service_links: Some(yes),
            ..self
        }
    }

    fn hostname(self, hostname: impl ToString) -> Self {
        let hostname = Some(hostname.to_string());
        Self { hostname, ..self }
    }

    fn host_ipc(self, yes: bool) -> Self {
        Self {
            host_ipc: Some(yes),
            ..self
        }
    }

    fn host_network(self, yes: bool) -> Self {
        Self {
            host_network: Some(yes),
            ..self
        }
    }

    fn host_pid(self, yes: bool) -> Self {
        Self {
            host_pid: Some(yes),
            ..self
        }
    }

    fn image_pull_secret(mut self, name: impl ToString) -> Self {
        let secret = corev1::LocalObjectReference::new(name);
        self.image_pull_secrets.get_or_insert_default().push(secret);
        self
    }

    fn node_name(self, node_name: impl ToString) -> Self {
        let node_name = Some(node_name.to_string());
        Self { node_name, ..self }
    }

    fn node_selector(
        mut self,
        node_selector: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let node_selector = node_selector
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));
        self.node_selector
            .get_or_insert_default()
            .extend(node_selector);
        self
    }

    fn preemption_policy(self, policy: impl ToString) -> Self {
        let preemption_policy = Some(policy.to_string());
        Self {
            preemption_policy,
            ..self
        }
    }

    fn priority(self, priority: i32) -> Self {
        Self {
            priority: Some(priority),
            ..self
        }
    }

    fn priority_class_name(self, class_name: impl ToString) -> Self {
        let priority_class_name = Some(class_name.to_string());
        Self {
            priority_class_name,
            ..self
        }
    }

    fn restart_policy(self, policy: impl ToString) -> Self {
        let restart_policy = Some(policy.to_string());
        Self {
            restart_policy,
            ..self
        }
    }

    fn runtime_class_name(self, class_name: impl ToString) -> Self {
        let runtime_class_name = Some(class_name.to_string());
        Self {
            runtime_class_name,
            ..self
        }
    }

    fn scheduler_name(self, scheduler: impl ToString) -> Self {
        let scheduler_name = Some(scheduler.to_string());
        Self {
            scheduler_name,
            ..self
        }
    }

    fn service_account_name(self, name: impl ToString) -> Self {
        let service_account_name = Some(name.to_string());
        Self {
            service_account_name,
            ..self
        }
    }

    fn service_account(self, name: impl ToString) -> Self {
        let service_account = Some(name.to_string());
        Self {
            service_account,
            ..self
        }
    }

    fn set_hostname_as_fqdn(self, yes: bool) -> Self {
        Self {
            set_hostname_as_fqdn: Some(yes),
            ..self
        }
    }

    fn share_process_namespace(self, yes: bool) -> Self {
        Self {
            share_process_namespace: Some(yes),
            ..self
        }
    }

    fn subdomain(self, subdomain: impl ToString) -> Self {
        let subdomain = Some(subdomain.to_string());
        Self { subdomain, ..self }
    }

    fn termination_grace_period_seconds(self, seconds: i64) -> Self {
        Self {
            termination_grace_period_seconds: Some(seconds),
            ..self
        }
    }

    fn tolerations(mut self, tolerations: impl IntoIterator<Item = corev1::Toleration>) -> Self {
        self.tolerations.get_or_insert_default().extend(tolerations);
        self
    }

    fn volumes(mut self, volumes: impl IntoIterator<Item = corev1::Volume>) -> Self {
        self.volumes.get_or_insert_default().extend(volumes);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_spec() -> corev1::PodSpec {
        corev1::PodSpec::container(corev1::Container::new("test-container"))
    }

    #[test]
    fn container() {
        let container = corev1::Container::new("test-container");
        let pod_spec = corev1::PodSpec::container(container);
        assert_eq!(pod_spec.containers.len(), 1);
        assert_eq!(pod_spec.containers[0].name, "test-container");
    }

    #[test]
    fn containers() {
        let containers_vec = vec![
            corev1::Container::new("container1"),
            corev1::Container::new("container2"),
        ];
        let pod_spec = corev1::PodSpec::containers(containers_vec);
        assert_eq!(pod_spec.containers.len(), 2);
        assert_eq!(pod_spec.containers[0].name, "container1");
        assert_eq!(pod_spec.containers[1].name, "container2");
    }

    #[test]
    fn active_deadline_seconds() {
        let pod_spec = test_spec().active_deadline_seconds(300);
        assert_eq!(pod_spec.active_deadline_seconds, Some(300));
    }

    #[test]
    fn affinity() {
        let affinity = corev1::Affinity::default();
        let pod_spec = test_spec().affinity(affinity);
        assert!(pod_spec.affinity.is_some());

        let pod_spec_none = test_spec().affinity(None);
        assert!(pod_spec_none.affinity.is_none());
    }

    #[test]
    fn automount_service_account_token() {
        let pod_spec = test_spec().automount_service_account_token(true);
        assert_eq!(pod_spec.automount_service_account_token, Some(true));

        let pod_spec_false = test_spec().automount_service_account_token(false);
        assert_eq!(pod_spec_false.automount_service_account_token, Some(false));
    }

    #[test]
    fn dns_policy() {
        let pod_spec = test_spec().dns_policy("ClusterFirst");
        assert_eq!(pod_spec.dns_policy.unwrap(), "ClusterFirst");
    }

    #[test]
    fn enable_service_links() {
        let pod_spec = test_spec().enable_service_links(false);
        assert_eq!(pod_spec.enable_service_links, Some(false));

        let pod_spec_true = test_spec().enable_service_links(true);
        assert_eq!(pod_spec_true.enable_service_links, Some(true));
    }

    #[test]
    fn hostname() {
        let pod_spec = test_spec().hostname("my-pod");
        assert_eq!(pod_spec.hostname.unwrap(), "my-pod");
    }

    #[test]
    fn host_ipc() {
        let pod_spec = test_spec().host_ipc(true);
        assert_eq!(pod_spec.host_ipc, Some(true));
    }

    #[test]
    fn host_network() {
        let pod_spec = test_spec().host_network(true);
        assert_eq!(pod_spec.host_network, Some(true));
    }

    #[test]
    fn host_pid() {
        let pod_spec = test_spec().host_pid(true);
        assert_eq!(pod_spec.host_pid, Some(true));
    }

    #[test]
    fn image_pull_secret() {
        let pod_spec = test_spec()
            .image_pull_secret("secret1")
            .image_pull_secret("secret2");

        let secrets = pod_spec.image_pull_secrets.unwrap_or_default();
        assert_eq!(secrets.len(), 2);
        // LocalObjectReference.name field is String
        assert_eq!(secrets[0].name, "secret1");
        assert_eq!(secrets[1].name, "secret2");
    }

    #[test]
    fn node_name() {
        let pod_spec = test_spec().node_name("worker-node-1");
        assert_eq!(pod_spec.node_name.unwrap(), "worker-node-1");
    }

    #[test]
    fn node_selector() {
        let pod = corev1::Pod::default().spec(
            test_spec().node_selector([("kubernetes.io/os", "linux"), ("node-type", "compute")]),
        );

        let node_selector = pod.node_selector().unwrap();
        assert_eq!(node_selector.len(), 2);
        assert_eq!(node_selector.get("kubernetes.io/os").unwrap(), "linux");
        assert_eq!(node_selector.get("node-type").unwrap(), "compute");
    }

    #[test]
    fn preemption_policy() {
        let pod_spec = test_spec().preemption_policy("Never");
        assert_eq!(pod_spec.preemption_policy.unwrap(), "Never");
    }

    #[test]
    fn priority() {
        let pod_spec = test_spec().priority(100);
        assert_eq!(pod_spec.priority, Some(100));
    }

    #[test]
    fn priority_class_name() {
        let pod_spec = test_spec().priority_class_name("high-priority");
        assert_eq!(pod_spec.priority_class_name.unwrap(), "high-priority");
    }

    #[test]
    fn restart_policy() {
        let pod_spec = test_spec().restart_policy("OnFailure");
        assert_eq!(pod_spec.restart_policy.unwrap(), "OnFailure");
    }

    #[test]
    fn runtime_class_name() {
        let pod_spec = test_spec().runtime_class_name("gvisor");
        assert_eq!(pod_spec.runtime_class_name.unwrap(), "gvisor");
    }

    #[test]
    fn scheduler_name() {
        let pod_spec = test_spec().scheduler_name("custom-scheduler");
        assert_eq!(pod_spec.scheduler_name.unwrap(), "custom-scheduler");
    }

    #[test]
    fn service_account_name() {
        let pod_spec = test_spec().service_account_name("my-service-account");
        assert_eq!(pod_spec.service_account_name.unwrap(), "my-service-account");
    }

    #[test]
    #[expect(deprecated)]
    fn service_account_deprecated() {
        let pod_spec = test_spec().service_account("deprecated-account");
        assert_eq!(pod_spec.service_account.unwrap(), "deprecated-account");
    }

    #[test]
    fn set_hostname_as_fqdn() {
        let pod_spec = test_spec().set_hostname_as_fqdn(true);
        assert_eq!(pod_spec.set_hostname_as_fqdn, Some(true));
    }

    #[test]
    fn share_process_namespace() {
        let pod_spec = test_spec().share_process_namespace(true);
        assert_eq!(pod_spec.share_process_namespace, Some(true));
    }

    #[test]
    fn subdomain() {
        let pod_spec = test_spec().subdomain("my-subdomain");
        assert_eq!(pod_spec.subdomain.unwrap(), "my-subdomain");
    }

    #[test]
    fn termination_grace_period_seconds() {
        let pod_spec = test_spec().termination_grace_period_seconds(60);
        assert_eq!(pod_spec.termination_grace_period_seconds, Some(60));
    }

    #[test]
    fn tolerations() {
        let toleration1 =
            corev1::Toleration::no_schedule("node-role.kubernetes.io/master").exists();
        let toleration2 = corev1::Toleration::no_execute("node.kubernetes.io/not-ready")
            .exists()
            .toleration_seconds(300);

        let pod = corev1::Pod::default().spec(test_spec().tolerations([toleration1, toleration2]));

        let tolerations = pod.tolerations().unwrap();
        assert_eq!(tolerations.len(), 2);
        let t0 = &tolerations[0];
        let t1 = &tolerations[1];

        assert_eq!(t0.key.as_ref().unwrap(), "node-role.kubernetes.io/master");
        assert_eq!(t0.operator.as_ref().unwrap(), "Exists");
        assert_eq!(t0.effect.as_ref().unwrap(), "NoSchedule");

        assert_eq!(t1.key.as_ref().unwrap(), "node.kubernetes.io/not-ready");
        assert_eq!(t1.operator.as_ref().unwrap(), "Exists");
        assert_eq!(t1.effect.as_ref().unwrap(), "NoExecute");
        assert_eq!(t1.toleration_seconds, Some(300));
    }

    #[test]
    fn volumes() {
        let volume1 = corev1::Volume {
            name: "config-vol".to_string(),
            config_map: Some(corev1::ConfigMapVolumeSource {
                name: "my-config".to_string(),
                ..default()
            }),
            ..default()
        };
        let volume2 = corev1::Volume {
            name: "empty-vol".to_string(),
            empty_dir: Some(corev1::EmptyDirVolumeSource::default()),
            ..default()
        };

        let pod_spec = test_spec().volumes([volume1, volume2]);

        let volumes = pod_spec.volumes.as_ref().unwrap();
        assert_eq!(volumes.len(), 2);
        assert_eq!(volumes[0].name, "config-vol");
        assert_eq!(volumes[1].name, "empty-vol");
        assert!(volumes[0].config_map.is_some());
        assert!(volumes[1].empty_dir.is_some());
    }

    #[test]
    fn method_chaining() {
        let container = corev1::Container::new("test-container");

        let pod_spec = corev1::PodSpec::container(container)
            .hostname("my-pod")
            .host_network(true)
            .dns_policy("ClusterFirst")
            .restart_policy("Always")
            .priority(100)
            .service_account_name("my-sa")
            .image_pull_secret("my-secret")
            .node_selector([("zone", "us-west-2a")])
            .termination_grace_period_seconds(30);

        assert_eq!(pod_spec.containers.len(), 1);
        assert_eq!(pod_spec.hostname.as_ref().unwrap(), "my-pod");
        assert_eq!(pod_spec.host_network, Some(true));
        assert_eq!(pod_spec.dns_policy.as_ref().unwrap(), "ClusterFirst");
        assert_eq!(pod_spec.restart_policy.as_ref().unwrap(), "Always");
        assert_eq!(pod_spec.priority, Some(100));
        assert_eq!(pod_spec.service_account_name.as_ref().unwrap(), "my-sa");
        assert_eq!(pod_spec.image_pull_secrets.as_ref().unwrap().len(), 1);

        // Use PodGetExt for node_selector - more ergonomic
        let pod = corev1::Pod::default().spec(pod_spec);
        assert_eq!(
            pod.node_selector().unwrap().get("zone").unwrap(),
            "us-west-2a"
        );
        assert_eq!(
            pod.spec.as_ref().unwrap().termination_grace_period_seconds,
            Some(30)
        );
    }
}
