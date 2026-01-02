// Modules
pub mod player_movement_controller;

use player_movement_controller::MoveDescriptor;

#[derive(Debug)]
pub enum PlayerIntention {
    Move(MoveDescriptor), 
}
