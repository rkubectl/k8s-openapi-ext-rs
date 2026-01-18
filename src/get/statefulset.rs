use super::*;

pub trait StatefulSetGetExt {
    fn spec(&self) -> Option<&appsv1::StatefulSetSpec>;

    fn status(&self) -> Option<&appsv1::StatefulSetStatus>;

    // From spec
    fn ordinals(&self) -> Option<i32> {
        self.spec()?.ordinals.as_ref()?.start
    }

    fn pod_management_policy(&self) -> Option<&str> {
        self.spec()?.pod_management_policy.as_deref()
    }

    fn revision_history_limit(&self) -> Option<i32> {
        self.spec()?.revision_history_limit
    }

    fn service_name(&self) -> Option<&str> {
        self.spec()?.service_name.as_deref()
    }

    fn spec_replicas(&self) -> Option<i32> {
        self.spec()?.replicas
    }

    fn min_ready_seconds(&self) -> Option<i32> {
        self.spec()?.min_ready_seconds
    }

    /// Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset.
    fn available_replicas(&self) -> Option<i32> {
        self.status()?.available_replicas
    }

    /// The count of hash collisions for the StatefulSet.
    /// The StatefulSet controller uses this field as a collision avoidance mechanism
    /// when it needs to create the name for the newest ControllerRevision.
    fn collision_count(&self) -> Option<i32> {
        self.status()?.collision_count
    }

    /// Represents the latest available observations of a statefulset’s current state.
    fn conditions(&self) -> Option<&[appsv1::StatefulSetCondition]> {
        self.status()?.conditions.as_deref()
    }

    /// The number of Pods created by the StatefulSet controller from the StatefulSet version indicated by `current_revision`.
    fn current_replicas(&self) -> Option<i32> {
        self.status()?.current_replicas
    }

    /// If not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas).
    fn current_revision(&self) -> Option<&str> {
        self.status()?.current_revision.as_deref()
    }

    /// The most recent generation observed for this StatefulSet.
    /// It corresponds to the StatefulSet’s generation, which is updated on mutation by the API Server.
    fn observed_generation(&self) -> Option<i64> {
        self.status()?.observed_generation
    }

    /// The number of pods created for this StatefulSet with a Ready Condition.
    fn ready_replicas(&self) -> Option<i32> {
        self.status()?.ready_replicas
    }

    /// The number of Pods created by the StatefulSet controller.
    fn status_replicas(&self) -> i32 {
        self.status()
            .map(|status| status.replicas)
            .unwrap_or_default()
    }

    /// If not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas)
    fn update_revision(&self) -> Option<&str> {
        self.status()?.update_revision.as_deref()
    }

    /// The number of Pods created by the StatefulSet controller from the StatefulSet version indicated by `update_revision`.
    fn updated_replicas(&self) -> Option<i32> {
        self.status()?.updated_replicas
    }
}

impl StatefulSetGetExt for appsv1::StatefulSet {
    fn spec(&self) -> Option<&appsv1::StatefulSetSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&appsv1::StatefulSetStatus> {
        self.status.as_ref()
    }
}
