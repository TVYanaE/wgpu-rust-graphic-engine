use std::{
    rc::Rc,
    cell::RefCell
};
use flume::{
    Sender,
};
use crate::{
    structures::{
        executeur_thread::{
            buses::{
                executeur_thread_message_bus::ExecuteurThreadMessageBus,
                executeur_thread_data_bus::ExecuteurThreadDataBus,
            },
        },
    },
    enums::{
        execute_thread_message_enums::{
            ExecuteurThreadGlobalExecuteurMessage
        },
        signals::{
            logic_thread_signal_enums::LogicThreadInputSignal,
            render_thread_signal_enums::RenderThreadInputSignal,
        },
        task_type_enum::TaskType,
    },
};

pub struct ExecuteurThreadGlobalExecuteur {
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
    executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
    logic_thread_input_channel_sender: Sender<LogicThreadInputSignal>,
    render_thread_input_channel_sender: Sender<RenderThreadInputSignal>,
}

impl ExecuteurThreadGlobalExecuteur {
    pub fn new(
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
        executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
        logic_thread_input_channel_sender: Sender<LogicThreadInputSignal>,
        render_thread_input_channel_sender: Sender<RenderThreadInputSignal>,
    ) -> Self {
        Self {
            executeur_thread_message_bus: executeur_thread_message_bus, 
            executeur_thread_data_bus: executeur_thread_data_bus, 
            logic_thread_input_channel_sender: logic_thread_input_channel_sender,
            render_thread_input_channel_sender: render_thread_input_channel_sender,
        }
    }

    pub fn start(&self) {
        let mut job_list_ready = false; 

        for message in self
            .executeur_thread_message_bus
            .borrow_mut()
            .drain_global_executeur_message_buffer() {
            match message {
                ExecuteurThreadGlobalExecuteurMessage::JobListReady => {
                    job_list_ready = true;
                },
            }
        }

        if !job_list_ready {
            return;
        }

        for mut task_chunk in self
            .executeur_thread_data_bus
            .borrow_mut()
            .drain_job_list() {
            for task in task_chunk.drain_chunk() {
                match task.task_type {                    
                    TaskType::Resize => {
                        self.logic_thread_input_channel_sender.send(LogicThreadInputSignal::Resize);
                        self.render_thread_input_channel_sender.send(RenderThreadInputSignal::Resize);
                    },
                    TaskType::LogicCalculation => {
                        self.logic_thread_input_channel_sender.send(LogicThreadInputSignal::LogicCalculation);
                    },
                    TaskType::PrepareRenderState => {
                        self.logic_thread_input_channel_sender.send(LogicThreadInputSignal::PrepareRenderState);
                    },
                    TaskType::DrawRenderState => {
                        self.render_thread_input_channel_sender.send(RenderThreadInputSignal::DrawRenderState);
                    },
                    TaskType::UnknowTask => {},
                }
            }
        }
    }
}
