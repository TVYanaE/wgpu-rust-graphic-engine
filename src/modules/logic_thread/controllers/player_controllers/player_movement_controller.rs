use glam::{
    Vec2, vec2,
};
use shipyard::{
    UniqueViewMut,
    UniqueView,
    View,
    IntoIter,
};
use super::{
    super::{
        super::{
            user_input::{
                action::Action,
                input_bindings::InputBindings,
                keyboard_state::KeyboardState,
            },
            components::{
                player_component::PlayerComponent,
            },
        },
        IntentionQuery, Intention, IntentionKind, 
    },
    PlayerIntention,
};

#[derive(Debug)]
pub struct MoveDescriptor {
    pub movement_vector: Vec2,
}

pub fn player_movement_controller(
    keyboard_state: UniqueView<KeyboardState>,
    input_bindings: UniqueView<InputBindings>,
    mut intention_query: UniqueViewMut<IntentionQuery>,
    view_player: View<PlayerComponent>,
) {
    for (player_id,_) in view_player.iter().with_id() {
        let move_up_bingings = input_bindings.get_action_bindings(&Action::MoveUp).unwrap(); 
        let move_left_bingings = input_bindings.get_action_bindings(&Action::MoveLeft).unwrap(); 
        let move_down_bingings = input_bindings.get_action_bindings(&Action::MoveDown).unwrap(); 
        let move_right_bingings = input_bindings.get_action_bindings(&Action::MoveRight).unwrap();

        let mut move_up = false;
        let mut move_left = false;
        let mut move_down = false;
        let mut move_right = false;

        for move_up_binging in move_up_bingings {
            if keyboard_state.is_pressed(move_up_binging) {
                move_up = true;
                break;
            } 
        }

        for move_left_binging in move_left_bingings {
            if keyboard_state.is_pressed(move_left_binging) {
                move_left = true;
                break;
            } 
        }

        for move_down_binging in move_down_bingings {
            if keyboard_state.is_pressed(move_down_binging) {
                move_down = true;
                break;
            } 
        }

        for move_right_binging in move_right_bingings {
            if keyboard_state.is_pressed(move_right_binging) {
                move_right = true;
                break;
            } 
        }

        let mut move_direction = vec2(0.0, 0.0);
        let magnitude: f32 = 0.01;

        if move_up {
            move_direction += vec2(0.0, magnitude);
        }

        if move_left {
            move_direction += vec2(-magnitude, 0.0);
        }

        if move_down {
            move_direction += vec2(0.0, -magnitude);
        }

        if move_right {
            move_direction += vec2(magnitude, 0.0);
        }

        let move_descriptor = MoveDescriptor {
            movement_vector: move_direction, 
        };

        let player_intention = PlayerIntention::Move(move_descriptor);

        let intention = Intention {
            kind: IntentionKind::Player(player_intention),
            entity_id_target: player_id, 
        };

        intention_query.0.push_back(intention);
    } 
}
