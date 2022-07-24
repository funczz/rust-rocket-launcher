use rust_fsm::{FsmState, FsmTransition};

use crate::{
    domain,
    fsm::{fsm_error::RocketLauncherFsmError, RocketLauncherEvent},
};

use super::END;

#[allow(nonstandard_style)]
#[derive(Debug, Clone)]
pub struct ABORTED;

impl FsmState for ABORTED {
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
            &RocketLauncherEvent::FINISH => Ok(FsmTransition::External(END.as_box())),
            _ => Err(RocketLauncherFsmError::EventNotFoundError {
                event: event.clone(),
            }),
        }
    }

    fn on_entry(&self, event: &Self::Event, ctx: Self::Ctx) -> Result<Self::Ctx, Self::Error> {
        if let &RocketLauncherEvent::ABORT = event {
            let mut new = ctx.clone();
            new.state = domain::RocketLauncherState::Aborted;
            Ok(new)
        } else {
            Ok(ctx)
        }
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
    fn event_not_found_start() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: None,
        };
        let result = state::ABORTED.fire(RocketLauncherEvent::START(10), ctx);
        match result {
            Ok((state, ctx)) => panic!("state={:?} ctx={:?}", state, ctx),
            Err(e) => assert_eq!(format!("{}", e), "event not found: START(10)"),
        }
    }

    #[test]
    fn event_not_found_decrement() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: None,
        };
        let result = state::ABORTED.fire(RocketLauncherEvent::DECREMENT, ctx);
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
        let result = state::ABORTED.fire(RocketLauncherEvent::LAUNCH, ctx);
        match result {
            Ok((state, ctx)) => panic!("state={:?} ctx={:?}", state, ctx),
            Err(e) => assert_eq!(format!("{}", e), "event not found: LAUNCH"),
        }
    }

    #[test]
    fn event_not_found_abort() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: None,
        };
        let result = state::ABORTED.fire(RocketLauncherEvent::ABORT, ctx);
        match result {
            Ok((state, ctx)) => panic!("state={:?} ctx={:?}", state, ctx),
            Err(e) => assert_eq!(format!("{}", e), "event not found: ABORT"),
        }
    }

    #[test]
    fn finish() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: Some(0),
        };
        let result = state::ABORTED.fire(RocketLauncherEvent::FINISH, ctx);
        match result {
            Ok((state, ctx)) => {
                assert_eq!(format!("{:?}", state), "END");
                assert_eq!(ctx.count.unwrap(), 0);
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
