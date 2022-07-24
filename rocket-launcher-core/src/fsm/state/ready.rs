use rust_fsm::{FsmState, FsmTransition};

use crate::{
    domain,
    fsm::{RocketLauncherEvent, RocketLauncherFsmError},
};

use super::{ABORTED, COUNTING};

#[allow(nonstandard_style)]
#[derive(Debug, Clone)]
pub struct READY;

impl FsmState for READY {
    type Event = RocketLauncherEvent;
    type Ctx = domain::RocketLauncherModel;
    type Error = RocketLauncherFsmError;

    fn as_box(
        &self,
    ) -> Box<dyn FsmState<Event = Self::Event, Ctx = Self::Ctx, Error = Self::Error>> {
        Box::new(Self)
    }

    fn to_transition(
        &self,
        event: &Self::Event,
    ) -> Result<FsmTransition<Self::Event, Self::Ctx, Self::Error>, Self::Error> {
        match event {
            &RocketLauncherEvent::START(_) => Ok(FsmTransition::External(COUNTING.as_box())),
            &RocketLauncherEvent::ABORT => Ok(FsmTransition::External(ABORTED.as_box())),
            _ => Err(RocketLauncherFsmError::EventNotFoundError {
                event: event.clone(),
            }),
        }
    }

    fn on_entry(&self, _event: &Self::Event, ctx: Self::Ctx) -> Result<Self::Ctx, Self::Error> {
        Ok(ctx)
    }

    fn on_do(&self, _event: &Self::Event, ctx: Self::Ctx) -> Result<Self::Ctx, Self::Error> {
        Ok(ctx)
    }

    fn on_exit(&self, _event: &Self::Event, ctx: Self::Ctx) -> Result<Self::Ctx, Self::Error> {
        Ok(ctx)
    }
}

#[cfg(test)]
mod tests {

    use rust_fsm::FsmState;

    use crate::{
        domain,
        fsm::{state, RocketLauncherEvent},
    };

    #[test]
    fn start() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: None,
        };
        let result = state::READY.fire(RocketLauncherEvent::START(10), ctx);
        match result {
            Ok((state, ctx)) => {
                assert_eq!(format!("{:?}", state), "COUNTING");
                assert_eq!(ctx.count.unwrap(), 10);
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn event_not_found_decrement() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: None,
        };
        let result = state::READY.fire(RocketLauncherEvent::DECREMENT, ctx);
        match result {
            Ok((state, ctx)) => panic!("state={:?} ctx={:?}", state, ctx),
            Err(e) => assert_eq!(format!("{}", e), "event not found: DECREMENT"),
        }
    }

    #[test]
    fn event_not_found_launch() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: None,
        };
        let result = state::READY.fire(RocketLauncherEvent::LAUNCH, ctx);
        match result {
            Ok((state, ctx)) => panic!("state={:?} ctx={:?}", state, ctx),
            Err(e) => assert_eq!(format!("{}", e), "event not found: LAUNCH"),
        }
    }

    #[test]
    fn abort() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: Some(10),
        };
        let result = state::READY.fire(RocketLauncherEvent::ABORT, ctx);
        match result {
            Ok((state, ctx)) => {
                assert_eq!(format!("{:?}", state), "ABORTED");
                assert_eq!(ctx.count.unwrap(), 10);
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn event_not_found_finish() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: None,
        };
        let result = state::READY.fire(RocketLauncherEvent::FINISH, ctx);
        match result {
            Ok((state, ctx)) => panic!("state={:?} ctx={:?}", state, ctx),
            Err(e) => assert_eq!(format!("{}", e), "event not found: FINISH"),
        }
    }
}
