use std::{
    sync::{Arc, RwLock},
    rc::Rc,
};
use crate::{
    structures::{
        scheduler::Scheduler,
        event_buffer_recorder::EventBufferRecorder,
        orchestrators::{
            global_orchestrator::GlobalOrchestrator,
            archetype_orchestrator::ArcherypeOrchestrator,
            system_orchestrator::SystemOrchestrator,
        },
        managers::{
            material_manager::MaterialManager
        },
        quartzes::global_quartz::GlobalQuartz,
        winit_event_recorder::WinitEventRecorder,
        winit_data_buffer::WinitDataBuffer,
    },
};

pub struct LogicState {
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
    scheduler: Arc<RwLock<Scheduler>>,
    global_quartz: GlobalQuartz,
    winit_event_recorder: Arc<RwLock<WinitEventRecorder>>,
    winit_data_buffer: Arc<RwLock<WinitDataBuffer>>,
    archetype_orchestrator: Arc<RwLock<ArcherypeOrchestrator>>,
    system_orchestrator: Arc<RwLock<SystemOrchestrator>>,
    global_orchestrator: GlobalOrchestrator,
}

impl LogicState {
    pub fn new(material_manager: Rc<MaterialManager>) -> Self {
        let event_buffer_recorder = Arc::new(RwLock::new(EventBufferRecorder::new()));
        let scheduler = Arc::new(RwLock::new(Scheduler::new(event_buffer_recorder.clone())));
       
        let winit_event_recorder = Arc::new(RwLock::new(WinitEventRecorder::new()));
        let winit_data_buffer = Arc::new(RwLock::new(WinitDataBuffer::new()));

        let global_quartz = GlobalQuartz::new(
            event_buffer_recorder.clone(), 
            winit_event_recorder.clone(), 
            winit_data_buffer.clone(),
        );
        
        let archetype_orchestrator = Arc::new(RwLock::new(ArcherypeOrchestrator::new()));
        let systen_orchestrator = Arc::new(RwLock::new(SystemOrchestrator::new(material_manager)));
        let global_orchestrator = GlobalOrchestrator::new(
            scheduler.clone(),
            systen_orchestrator.clone(),
            archetype_orchestrator.clone(),
        );

        Self {
            event_buffer_recorder: event_buffer_recorder,
            scheduler: scheduler,
            winit_data_buffer: winit_data_buffer,
            winit_event_recorder: winit_event_recorder,
            global_quartz: global_quartz,
            archetype_orchestrator: archetype_orchestrator,
            system_orchestrator: systen_orchestrator,
            global_orchestrator: global_orchestrator,
        }
    }

    pub fn run_tact(&mut self) {
         
    }
}


