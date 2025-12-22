use std::{
    rc::Rc,
    cell::RefCell,
    sync::{Arc, Mutex},
};
use flume::{
    Receiver, TryRecvError,
};
use crate::{
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
        },
        phase_enum::Phase, 
        event_enum::Event,
        task_type_enum::TaskType,
        errors::ControlThreadPhaseManagerError,
    },
    structures::{
        buses::{
            control_thread_message_bus::ControlThreadMessagesBus,
            io_bus::IOBus,
        },
        states::{
            phase_state::PhaseState,
        },
        task::Task,
        phase_priority::PhasePriority,
    },
};

pub struct ControlThreadPhaseManager {
    phase_state: Rc<RefCell<PhaseState>>,
    io_bus: Arc<Mutex<IOBus>>,
    signals_debt: Vec<ControlThreadInputSignal>,
    control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
    control_thread_message_bus_ref: Rc<RefCell<ControlThreadMessagesBus>>,
}

impl ControlThreadPhaseManager {
    pub fn new(
        phase_state: Rc<RefCell<PhaseState>>,
        io_bus: Arc<Mutex<IOBus>>,
        control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
        control_thread_message_bus_ref: Rc<RefCell<ControlThreadMessagesBus>>,
    ) -> Self {
        Self { 
            phase_state: phase_state,
            io_bus: io_bus,
            signals_debt: Vec::new(),
            control_thread_input_channel_receiver: control_thread_input_channel_receiver, 
            control_thread_message_bus_ref: control_thread_message_bus_ref,
        }
    }

    fn select_phase(signals: &[ControlThreadInputSignal]) -> Phase {
        // TODO:!!!!! Stopped here
    }

    pub fn start(&mut self) -> Result<(), ControlThreadPhaseManagerError> {
        let mut message_bus = self.control_thread_message_bus_ref.borrow_mut();
        let mut phase_state = self.phase_state.borrow_mut(); 
        let prev_phase = phase_state.get_prev_phase();
 
        let mut signals = Vec::new();

        loop {
            match self.control_thread_input_channel_receiver.try_recv() {
                Ok(control_thread_input_signal) => {
                    match control_thread_input_signal {
                        ControlThreadInputSignal::FrameStart => {
                            if !signals.contains(&ControlThreadInputSignal::FrameStart) {
                                signals.push(control_thread_input_signal);
                            }
                            else {
                                continue;
                            }
                        },
                        _ => {
                            signals.push(control_thread_input_signal);
                        },
                    } 
                },
                Err(try_recv_error) => {
                    match try_recv_error {
                        TryRecvError::Empty => { break; },
                        TryRecvError::Disconnected => { 
                            return Err(ControlThreadPhaseManagerError::ChannelClosedError(try_recv_error)); 
                        }
                    }
                },
            }
        }

        if signals.is_empty() {
            // logic for IO events
            // and if IO Buffer empty then Phase Idle 
        }

        for signal in signals {
            match signal {
                ControlThreadInputSignal::Init => { 
                    phase_state.set_current_phase(Phase::InitPhase); 

                    let task = Task {
                        work_phase: Phase::InitPhase,
                        task_type: TaskType::Init,
                    };

                    message_bus.push_task_to_bus(task);
                },
                ControlThreadInputSignal::Shutdown => {
                    phase_state.set_current_phase(Phase::ShutdownPhase);

                    let task = Task {
                        work_phase: Phase::ShutdownPhase,
                        task_type: TaskType::Shutdown,
                    };

                    message_bus.push_task_to_bus(task);
                },
                ControlThreadInputSignal::LogicStart => {
                    phase_state.set_current_phase(Phase::UpdatePhase);

                    let task_1 = Task {
                        work_phase: Phase::UpdatePhase,
                        task_type: TaskType::LogicCalculation,
                    };

                    let task_2 = Task {
                        work_phase: Phase::UpdatePhase,
                        task_type: TaskType::PrepareRenderState,
                    };
                    
                    message_bus.push_task_to_bus(task_1);  
                    message_bus.push_task_to_bus(task_2); 
                },
                ControlThreadInputSignal::FrameStart => {
                    phase_state.set_current_phase(Phase::RenderPhase);
                    
                    let task = Task {
                        work_phase: Phase::RenderPhase,
                        task_type: TaskType::DrawRenderState,
                    };
                    
                    message_bus.push_task_to_bus(task);
                },
            }
        } 

        return Ok(());
    }
}
