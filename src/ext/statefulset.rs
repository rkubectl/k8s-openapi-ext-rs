use super::*;

pub trait StatefulSetExt: super::ResourceBuilder {
    /// Create a new StatefulSet with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset");
    /// assert_eq!(statefulset.metadata.name, Some("my-statefulset".to_string()));
    /// ```
    fn new(name: impl ToString) -> Self;

    /// Create a new StatefulSet with the given name and labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::with_labels(
    ///     "my-statefulset",
    ///     [("app", "my-app"), ("version", "v1")]
    /// );
    /// ```
    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    /// Set the spec for the StatefulSet.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let spec = appsv1::StatefulSetSpec::default();
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .spec(spec);
    /// ```
    fn spec(self, spec: appsv1::StatefulSetSpec) -> Self;

    /// Set the number of replicas for the StatefulSet.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .replicas(3);
    /// ```
    fn replicas(self, replicas: i32) -> Self;

    /// Set the minimum ready seconds for the StatefulSet.
    ///
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing, for it to be considered available.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .min_ready_seconds(30);
    /// ```
    fn min_ready_seconds(self, seconds: i32) -> Self;

    /// Set the selector for the StatefulSet.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::metav1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let selector = metav1::LabelSelector::default();
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .selector(selector);
    /// ```
    fn selector(self, selector: metav1::LabelSelector) -> Self;

    /// Set the matchLabels for the StatefulSet's selector.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .match_labels([("app", "my-app"), ("version", "v1")]);
    /// ```
    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    /// Set the service name for the StatefulSet.
    ///
    /// The service name is used to govern the StatefulSet and must be the name
    /// of an existing headless service.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .service_name("my-headless-service");
    /// ```
    fn service_name(self, service_name: impl ToString) -> Self;

    /// Set the revision history limit for the StatefulSet.
    ///
    /// The number of old ReplicaSets to retain to allow rollback.
    /// This is a pointer to distinguish between explicit zero and not specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .revision_history_limit(5);
    /// ```
    fn revision_history_limit(self, limit: i32) -> Self;

    /// Set the pod template spec for the StatefulSet.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::corev1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let template = corev1::PodTemplateSpec::default();
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .template(template);
    /// ```
    fn template(self, template: corev1::PodTemplateSpec) -> Self;

    /// Set the starting ordinal for the StatefulSet.
    ///
    /// Controls the starting ordinal number for the StatefulSet pods.
    /// By default, pods start from ordinal 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .ordinals(1); // Pods will start from ordinal 1
    /// ```
    fn ordinals(self, ordinals: i32) -> Self;

    /// Set the update strategy for the StatefulSet.
    ///
    /// Update strategy indicates the StatefulSetUpdateStrategy that will be
    /// employed to update pods when a revision is made to the template.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let strategy = appsv1::StatefulSetUpdateStrategy::default();
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .update_strategy(strategy);
    /// ```
    fn update_strategy(self, strategy: appsv1::StatefulSetUpdateStrategy) -> Self;

    /// Set the pod management policy for the StatefulSet.
    ///
    /// Pod management policy controls how pods are created during initial scale up,
    /// replacement, or scale down operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .pod_management_policy("Parallel");
    /// ```
    fn pod_management_policy(self, policy: impl ToString) -> Self;

    /// Set the pod management policy to "OrderedReady" (default).
    ///
    /// In OrderedReady mode, pods are created, scaled, and deleted in order.
    /// Each pod must be Ready before the next one is created or deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .ordered_ready();
    /// ```
    fn ordered_ready(self) -> Self;

    /// Set the pod management policy to "Parallel".
    ///
    /// In Parallel mode, pods are created and deleted in parallel,
    /// similar to Deployment behavior. Pods can start simultaneously
    /// without waiting for others to be Ready.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .parallel();
    /// ```
    fn parallel(self) -> Self;

    /// Set the volume claim templates for the StatefulSet.
    ///
    /// VolumeClaimTemplates is a list of claims that pods are allowed to reference.
    /// The StatefulSet controller is responsible for mapping network identities to
    /// claims in a way that maintains the identity of a pod.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::corev1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let claim = corev1::PersistentVolumeClaim::default();
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .volume_claim_templates([claim]);
    /// ```
    fn volume_claim_templates(
        self,
        templates: impl IntoIterator<Item = corev1::PersistentVolumeClaim>,
    ) -> Self;

    /// Set the persistent volume claim retention policy for the StatefulSet.
    ///
    /// PersistentVolumeClaimRetentionPolicy describes the policy used for PVCs
    /// created from the StatefulSet VolumeClaimTemplates.
    ///
    /// # Examples
    ///
    /// ```
    /// use k8s_openapi_ext::appsv1;
    /// use k8s_openapi_ext::StatefulSetExt;
    ///
    /// let policy = appsv1::StatefulSetPersistentVolumeClaimRetentionPolicy::default();
    /// let statefulset = appsv1::StatefulSet::new("my-statefulset")
    ///     .persistent_volume_claim_retention_policy(policy);
    /// ```
    fn persistent_volume_claim_retention_policy(
        self,
        policy: appsv1::StatefulSetPersistentVolumeClaimRetentionPolicy,
    ) -> Self;
}

impl StatefulSetExt for appsv1::StatefulSet {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // spec: todo!(),
            // status: todo!(),
            ..default()
        }
    }

    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        Self::new(name).labels(labels)
    }

    fn spec(self, spec: appsv1::StatefulSetSpec) -> Self {
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn replicas(mut self, replicas: i32) -> Self {
        self.spec_mut().replicas.replace(replicas);
        self
    }

    fn min_ready_seconds(mut self, seconds: i32) -> Self {
        self.spec_mut().min_ready_seconds.replace(seconds);
        self
    }

    fn selector(mut self, selector: metav1::LabelSelector) -> Self {
        self.spec_mut().selector = selector;
        self
    }

    fn match_labels(
        mut self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        self.spec_mut().selector = metav1::LabelSelector::match_labels(match_labels);
        self
    }

    fn service_name(mut self, service_name: impl ToString) -> Self {
        let service_name = service_name.to_string();
        self.spec_mut().service_name.replace(service_name);
        self
    }

    fn revision_history_limit(mut self, limit: i32) -> Self {
        self.spec_mut().revision_history_limit.replace(limit);
        self
    }

    fn template(mut self, template: corev1::PodTemplateSpec) -> Self {
        self.spec_mut().template = template;
        self
    }

    fn ordinals(mut self, start: i32) -> Self {
        self.spec_mut().ordinals = Some(appsv1::StatefulSetOrdinals { start: Some(start) });
        self
    }

    fn update_strategy(mut self, strategy: appsv1::StatefulSetUpdateStrategy) -> Self {
        self.spec_mut().update_strategy.replace(strategy);
        self
    }

    fn pod_management_policy(mut self, policy: impl ToString) -> Self {
        let policy = policy.to_string();
        self.spec_mut().pod_management_policy.replace(policy);
        self
    }

    fn ordered_ready(self) -> Self {
        self.pod_management_policy("OrderedReady")
    }

    fn parallel(self) -> Self {
        self.pod_management_policy("Parallel")
    }

    fn volume_claim_templates(
        mut self,
        templates: impl IntoIterator<Item = corev1::PersistentVolumeClaim>,
    ) -> Self {
        let templates = templates.into_iter().collect();
        self.spec_mut().volume_claim_templates.replace(templates);
        self
    }

    fn persistent_volume_claim_retention_policy(
        mut self,
        policy: appsv1::StatefulSetPersistentVolumeClaimRetentionPolicy,
    ) -> Self {
        self.spec_mut()
            .persistent_volume_claim_retention_policy
            .replace(policy);
        self
    }
}

impl HasSpec for appsv1::StatefulSet {
    type Spec = appsv1::StatefulSetSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
