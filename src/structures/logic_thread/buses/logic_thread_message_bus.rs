
use crate::{
    enums::{
        messages::{
            logic_thread_message_enums::{
                LogicThreadWorldManagerMessage, 
                LogicThreadExtractHandlerMessage,
                LogicThreadPrelogicHandlerMessage,
                LogicThreadPostlogicHandlerMessage,
                LogicThreadSimulationHandlerMessage,
                LogicThreadGameObjectManagerMessage,
                LogicThreadSpawnDespawnHandlerMessage,
                LogicThreadExternalInputHandlerMessage, 
            },
        },
    },
};

pub struct LogicThreadMessageBus {
    logic_thread_world_manager_messages: Vec<LogicThreadWorldManagerMessage>,
    logic_thread_extract_handler_messages: Vec<LogicThreadExtractHandlerMessage>,
    logic_thread_prelogic_handler_messages: Vec<LogicThreadPrelogicHandlerMessage>,
    logic_thread_postlogic_handler_messages: Vec<LogicThreadPostlogicHandlerMessage>,
    logic_thread_simulation_handler_messages: Vec<LogicThreadSimulationHandlerMessage>,
    logic_thread_game_object_manager_messages: Vec<LogicThreadGameObjectManagerMessage>,
    logic_thread_spawn_despawn_handler_messages: Vec<LogicThreadSpawnDespawnHandlerMessage>,
    logic_thread_external_input_handler_messages: Vec<LogicThreadExternalInputHandlerMessage>, 
}

impl LogicThreadMessageBus {
    pub fn new() -> Self {
        Self {
            logic_thread_world_manager_messages: Vec::new(),
            logic_thread_extract_handler_messages: Vec::new(),
            logic_thread_prelogic_handler_messages: Vec::new(),
            logic_thread_postlogic_handler_messages: Vec::new(),
            logic_thread_simulation_handler_messages: Vec::new(),
            logic_thread_game_object_manager_messages: Vec::new(),
            logic_thread_spawn_despawn_handler_messages: Vec::new(),
            logic_thread_external_input_handler_messages: Vec::new(),
        }
    }

    pub fn push_world_manager_message_to_bus(
        &mut self, 
        message: LogicThreadWorldManagerMessage
    ) {
        self.logic_thread_world_manager_messages.push(message);
    }

    pub fn push_extract_handler_message_to_bus(
        &mut self, 
        message: LogicThreadExtractHandlerMessage
    ) {
        self.logic_thread_extract_handler_messages.push(message);
    }

    pub fn push_prelogic_handler_message_to_bus(
        &mut self, 
        message: LogicThreadPrelogicHandlerMessage
    ) {
        self.logic_thread_prelogic_handler_messages.push(message);
    }

    pub fn push_postlogic_handler_message_to_bus(
        &mut self, 
        message: LogicThreadPostlogicHandlerMessage
    ) {
        self.logic_thread_postlogic_handler_messages.push(message);
    }

    pub fn push_simulation_handler_message_to_bus(
        &mut self, 
        message: LogicThreadSimulationHandlerMessage
    ) {
        self.logic_thread_simulation_handler_messages.push(message);
    }

    pub fn push_game_object_manager_message_to_bus(
        &mut self, 
        message: LogicThreadGameObjectManagerMessage
    ) {
        self.logic_thread_game_object_manager_messages.push(message);
    }

    pub fn push_spawn_despawn_handler_message_to_bus(
        &mut self, 
        message: LogicThreadSpawnDespawnHandlerMessage
    ) {
        self.logic_thread_spawn_despawn_handler_messages.push(message);
    }

    pub fn push_external_input_handler_message_to_bus(
        &mut self, 
        message: LogicThreadExternalInputHandlerMessage
    ) {
        self.logic_thread_external_input_handler_messages.push(message);
    }

    pub fn drain_world_manager_message_buffer(
        &mut self
    ) -> impl Iterator<Item = LogicThreadWorldManagerMessage > {
        self.logic_thread_world_manager_messages.drain(..)
    }
    
    pub fn drain_extract_handler_message_buffer(
        &mut self
    ) -> impl Iterator<Item = LogicThreadExtractHandlerMessage> {
        self.logic_thread_extract_handler_messages.drain(..)
    }

    pub fn drain_prelogic_handler_message_buffer(
        &mut self
    ) -> impl Iterator<Item = LogicThreadPrelogicHandlerMessage> {
        self.logic_thread_prelogic_handler_messages.drain(..)
    }

    pub fn drain_postlogic_handler_message_buffer(
        &mut self
    ) -> impl Iterator<Item = LogicThreadPostlogicHandlerMessage> {
        self.logic_thread_postlogic_handler_messages.drain(..)
    }

    pub fn drain_simulation_handler_message_buffer(
        &mut self
    ) -> impl Iterator<Item = LogicThreadSimulationHandlerMessage> {
        self.logic_thread_simulation_handler_messages.drain(..)
    }

    pub fn drain_game_object_manager_message_buffer(
        &mut self
    ) -> impl Iterator<Item = LogicThreadGameObjectManagerMessage> {
        self.logic_thread_game_object_manager_messages.drain(..)
    }

    pub fn drain_spawn_despawn_handler_message_buffer(
        &mut self
    ) -> impl Iterator<Item = LogicThreadSpawnDespawnHandlerMessage> {
        self.logic_thread_spawn_despawn_handler_messages.drain(..)
    }

    pub fn drain_external_input_handler_message_buffer(
        &mut self
    ) -> impl Iterator<Item = LogicThreadExternalInputHandlerMessage> {
        self.logic_thread_external_input_handler_messages.drain(..)
    }
}
