// Sub Modules
pub(super) mod player_controllers;

// Modules
pub(super) mod ai_controller;
pub(super) mod network_controller;

use std::{
    collections::VecDeque,
};
use shipyard::{
    EntityId, Unique,
};
use player_controllers::PlayerIntention;
use network_controller::NetworkIntention;
use ai_controller::AIIntention;

#[derive(Debug,)]
pub(super) enum IntentionKind {
    Player(PlayerIntention),
    Network(NetworkIntention),
    AI(AIIntention),
}

#[derive(Debug)]
pub(super) struct Intention {
    pub entity_id_target: EntityId,
    pub kind: IntentionKind,
}

#[derive(Debug, Unique)]
pub(super) struct IntentionQuery(pub VecDeque<Intention>);

impl Default for IntentionQuery {
    fn default() -> Self {
        Self(VecDeque::with_capacity(64))
    }
}
