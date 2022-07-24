use rust_fsm::FsmState;

use crate::{
    domain,
    fsm::{RocketLauncherEvent, RocketLauncherFsmError},
};

#[allow(nonstandard_style)]
#[derive(Debug, Clone)]
pub struct END;

impl FsmState for END {
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
    ) -> Result<rust_fsm::FsmTransition<Self::Event, Self::Ctx, Self::Error>, Self::Error> {
        Err(RocketLauncherFsmError::EventNotFoundError {
            event: event.clone(),
        })
    }

    fn on_entry(&self, event: &Self::Event, ctx: Self::Ctx) -> Result<Self::Ctx, Self::Error> {
        if let &RocketLauncherEvent::FINISH = event {
            let mut new = ctx.clone();
            new.state = domain::RocketLauncherState::End;
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
        let result = state::END.fire(RocketLauncherEvent::START(10), ctx);
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
        let result = state::END.fire(RocketLauncherEvent::DECREMENT, ctx);
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
        let result = state::END.fire(RocketLauncherEvent::LAUNCH, ctx);
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
        let result = state::END.fire(RocketLauncherEvent::ABORT, ctx);
        match result {
            Ok((state, ctx)) => panic!("state={:?} ctx={:?}", state, ctx),
            Err(e) => assert_eq!(format!("{}", e), "event not found: ABORT"),
        }
    }

    #[test]
    fn event_not_found_finish() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Ready,
            count: None,
        };
        let result = state::END.fire(RocketLauncherEvent::FINISH, ctx);
        match result {
            Ok((state, ctx)) => panic!("state={:?} ctx={:?}", state, ctx),
            Err(e) => assert_eq!(format!("{}", e), "event not found: FINISH"),
        }
    }

    #[test]
    fn finish_from_launched() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Launched,
            count: Some(0),
        };
        let result = state::LAUNCHED.fire(RocketLauncherEvent::FINISH, ctx);
        match result {
            Ok((state, ctx)) => {
                assert_eq!(format!("{:?}", state), "END");
                assert_eq!(ctx.count.unwrap(), 0);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn finish_from_aborted() {
        let ctx = domain::RocketLauncherModel {
            state: domain::RocketLauncherState::Aborted,
            count: Some(0),
        };
        let result = state::ABORTED.fire(RocketLauncherEvent::FINISH, ctx);
        match result {
            Ok((state, ctx)) => {
                assert_eq!(format!("{:?}", state), "END");
                assert_eq!(ctx.count.unwrap(), 0);
            }
            Err(e) => panic!("{}", e),
        }
    }
}
