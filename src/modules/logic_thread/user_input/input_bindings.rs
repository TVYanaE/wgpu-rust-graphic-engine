use std::{
    collections::{HashMap},
};
use shipyard::{
    Unique
};
use winit::{
    keyboard::KeyCode,
};
use super::{
    action::Action,
};

#[derive(Debug, Unique)]
pub struct InputBindings {
    actions: HashMap<Action, Vec<KeyCode>> 
}

impl Default for InputBindings {
    fn default() -> Self {
        let mut actions = HashMap::with_capacity(32);

        actions.insert(Action::MoveUp, vec![KeyCode::KeyW]);
        actions.insert(Action::MoveLeft, vec![KeyCode::KeyA]);
        actions.insert(Action::MoveDown, vec![KeyCode::KeyS]);
        actions.insert(Action::MoveRight, vec![KeyCode::KeyD]);

        Self::new(actions)
    }
}

impl InputBindings {
    pub fn new(actions: HashMap<Action, Vec<KeyCode>>) -> Self {
        
        Self {
            actions: actions,    
        }
    }

    pub fn get_action_bindings(&self, action: &Action) -> Option<&Vec<KeyCode>> {
        self.actions.get(action)
    }
}
