use std::{
    rc::Rc,
    cell::RefCell,
};
use shipyard::{
    World,
};
use crate::{
    structures::{
        buses::{
            ecs_thread_message_bus::ECSThreadMessageBus,
        },
    },
    enums::{
        ecs_thread_message_enums::{
            ECSThreadWorldManagerMessage,
        },
    },
};


pub struct WorldManager {
    ecs_thread_message_bus: Rc<RefCell<ECSThreadMessageBus>>, 
    world: Option<World>,
}

impl WorldManager {
    pub fn new(
        ecs_thread_message_bus: Rc<RefCell<ECSThreadMessageBus>>
    ) -> Self {
        Self { 
            ecs_thread_message_bus: ecs_thread_message_bus,
            world: None,
        }
    }

    pub fn start(&mut self) {
        for message in self
            .ecs_thread_message_bus
            .borrow_mut()
            .drain_world_manager_message_buffer() {
            match message {
                ECSThreadWorldManagerMessage::Init => {
                    let world = World::default();
                     
                    self.world = Some(world);
                },
                ECSThreadWorldManagerMessage::Shutdown => {

                },
                ECSThreadWorldManagerMessage::Resize => {

                },
                ECSThreadWorldManagerMessage::LogicCalculation => {

                },
                ECSThreadWorldManagerMessage::PrepareRenderState => {

                },
            }
        }
    }
}
