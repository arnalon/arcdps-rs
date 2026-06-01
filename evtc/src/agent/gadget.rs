use super::Visibility;
use crate::{AgentId, Event, StateChange, TryExtract, extract::Extract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Gadget is playing model animation.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GadgetAnimation {
    /// Time of registering the event.
    pub time: u64,

    /// Gadget agent that is playing the animation.
    pub agent: AgentId,

    /// Animation token.
    pub token: u64,
}

impl Extract for GadgetAnimation {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            token: event.dst_agent,
        }
    }
}

impl TryExtract for GadgetAnimation {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::GadgetAnimation
    }
}

/// Gadget name changed visibility.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GadgetName {
    /// Time of registering the event.
    pub time: u64,

    /// Gadget agent that changed name visibility.
    pub agent: AgentId,

    /// New name visibility.
    pub visible: Visibility,
}

impl Extract for GadgetName {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            visible: (event.dst_agent as u32).into(),
        }
    }
}

impl TryExtract for GadgetName {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::GadgetName
    }
}
