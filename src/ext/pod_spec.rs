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
