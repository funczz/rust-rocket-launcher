use crate::{domain, fsm};

impl
    Into<
        Box<
            dyn rust_fsm::FsmState<
                Event = fsm::RocketLauncherEvent,
                Ctx = domain::RocketLauncherModel,
                Error = fsm::RocketLauncherFsmError,
            >,
        >,
    > for domain::RocketLauncherState
{
    fn into(
        self,
    ) -> Box<
        dyn rust_fsm::FsmState<
            Event = fsm::RocketLauncherEvent,
            Ctx = domain::RocketLauncherModel,
            Error = fsm::RocketLauncherFsmError,
        >,
    > {
        match self {
            domain::RocketLauncherState::Aborted => Box::new(fsm::ABORTED),
            domain::RocketLauncherState::Counting => Box::new(fsm::COUNTING),
            domain::RocketLauncherState::End => Box::new(fsm::END),
            domain::RocketLauncherState::Launched => Box::new(fsm::LAUNCHED),
            domain::RocketLauncherState::Ready => Box::new(fsm::READY),
        }
    }
}
