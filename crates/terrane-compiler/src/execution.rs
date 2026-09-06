use crate::package::ExecutorProfile;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ExecutionRequirements {
    pub runtime_context: bool,
    pub wake_support: bool,
    pub local_task: bool,
    pub transferable_task: bool,
    pub blocking_delegation: bool,
}

impl ExecutionRequirements {
    pub(crate) fn merge(&mut self, other: Self) {
        self.runtime_context |= other.runtime_context;
        self.wake_support |= other.wake_support;
        self.local_task |= other.local_task;
        self.transferable_task |= other.transferable_task;
        self.blocking_delegation |= other.blocking_delegation;
    }

    pub(crate) fn is_consistent(self) -> bool {
        (!self.runtime_context || self.wake_support)
            && (!(self.local_task || self.transferable_task) || self.wake_support)
            && (!self.blocking_delegation || self.runtime_context)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExecutionStrategy {
    Local,
    Parallel,
}

impl ExecutionStrategy {
    pub(crate) fn from_profile(profile: ExecutorProfile) -> Self {
        match profile {
            ExecutorProfile::Cooperative => Self::Local,
            ExecutorProfile::Threaded => Self::Parallel,
        }
    }
}
