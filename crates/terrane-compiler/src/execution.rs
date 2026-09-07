use crate::package::ExecutorProfile;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ExecutionRequirements {
    pub runtime: RuntimeRequirements,
    pub tasks: TaskRequirements,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct RuntimeRequirements {
    pub context: bool,
    pub wake_support: bool,
    pub blocking_delegation: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct TaskRequirements {
    pub local: bool,
    pub transferable: bool,
}

impl ExecutionRequirements {
    pub(crate) fn for_async(local: bool) -> Self {
        Self {
            runtime: RuntimeRequirements {
                wake_support: true,
                ..RuntimeRequirements::default()
            },
            tasks: TaskRequirements {
                local,
                transferable: !local,
            },
        }
    }

    pub(crate) fn merge(&mut self, other: Self) {
        self.runtime.context |= other.runtime.context;
        self.runtime.wake_support |= other.runtime.wake_support;
        self.runtime.blocking_delegation |= other.runtime.blocking_delegation;
        self.tasks.local |= other.tasks.local;
        self.tasks.transferable |= other.tasks.transferable;
    }

    pub(crate) fn is_consistent(self) -> bool {
        if (self.runtime.context || self.tasks.local || self.tasks.transferable)
            && !self.runtime.wake_support
        {
            return false;
        }
        !self.runtime.blocking_delegation || self.runtime.context
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
