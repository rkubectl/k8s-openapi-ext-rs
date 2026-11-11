use super::*;

pub trait DeploymentGetExt {
    fn spec(&self) -> &appsv1::DeploymentSpec;

    fn status(&self) -> Option<&appsv1::DeploymentStatus>;

    // From spec

    fn min_ready_seconds(&self) -> Option<i32> {
        self.spec().min_ready_seconds
    }

    fn paused(&self) -> Option<bool> {
        self.spec().paused
    }

    fn progress_deadline_seconds(&self) -> Option<i32> {
        self.spec().progress_deadline_seconds
    }

    fn revision_history_limit(&self) -> Option<i32> {
        self.spec().revision_history_limit
    }

    fn selector(&self) -> &metav1::LabelSelector {
        &self.spec().selector
    }

    fn strategy(&self) -> Option<&appsv1::DeploymentStrategy> {
        self.spec().strategy.as_ref()
    }

    fn template(&self) -> &corev1::PodTemplateSpec {
        &self.spec().template
    }

    fn spec_replicas(&self) -> Option<i32> {
        self.spec().replicas
    }

    // From status
    fn conditions(&self) -> Option<&[appsv1::DeploymentCondition]> {
        self.status()?.conditions.as_deref()
    }

    fn status_replicas(&self) -> Option<i32> {
        self.status()?.replicas
    }

    fn available_replicas(&self) -> Option<i32> {
        self.status()?.available_replicas
    }

    fn ready_replicas(&self) -> Option<i32> {
        self.status()?.ready_replicas
    }

    fn unavailable_replicas(&self) -> Option<i32> {
        self.status()?.unavailable_replicas
    }

    fn updated_replicas(&self) -> Option<i32> {
        self.status()?.updated_replicas
    }

    fn collision_count(&self) -> Option<i32> {
        self.status()?.collision_count
    }

    fn observed_generation(&self) -> Option<i64> {
        self.status()?.observed_generation
    }

    // Shortcuts
    fn containers(&self) -> Option<&[corev1::Container]> {
        self.template().containers()
    }

    fn port_by_name(&self, name: impl AsRef<str>) -> Option<&corev1::ContainerPort> {
        let name = name.as_ref();
        self.containers()?
            .iter()
            .find_map(|container| container.port_by_name(name))
    }
}

impl DeploymentGetExt for appsv1::Deployment {
    fn spec(&self) -> &appsv1::DeploymentSpec {
        &self.spec
    }

    fn status(&self) -> Option<&appsv1::DeploymentStatus> {
        self.status.as_ref()
    }
}

impl DeploymentGetExt for appsv1::DeploymentSpec {
    fn spec(&self) -> &appsv1::DeploymentSpec {
        self
    }

    fn status(&self) -> Option<&appsv1::DeploymentStatus> {
        None
    }
}

impl DeploymentGetExt for appsv1::DeploymentStatus {
    fn spec(&self) -> &appsv1::DeploymentSpec {
        &EMPTY_DEPLOYMENT_SPEC
    }

    fn status(&self) -> Option<&appsv1::DeploymentStatus> {
        Some(self)
    }
}

static EMPTY_DEPLOYMENT_SPEC: appsv1::DeploymentSpec = appsv1::DeploymentSpec {
    min_ready_seconds: None,
    paused: None,
    progress_deadline_seconds: None,
    replicas: None,
    revision_history_limit: None,
    selector: metav1::LabelSelector {
        match_expressions: None,
        match_labels: None,
    },
    strategy: None,
    template: corev1::PodTemplateSpec {
        metadata: None,
        spec: None,
    },
};
