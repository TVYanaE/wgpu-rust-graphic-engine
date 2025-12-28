use std::{
    rc::Rc,
    cell::RefCell
};
use flume::{
    Sender,
};
use crate::{
    structures::{
        buses::{
            executeur_thread_message_bus::ExecuteurThreadMessageBus,
            executeur_thread_data_bus::ExecuteurThreadDataBus,
        },
    },
    enums::{
        execute_thread_message_enums::{
            ExecuteurThreadGlobalExecuteurMessage
        },
        signals::{
            ecs_thread_signal_enums::ECSThreadInputSignal,
            render_thread_signal_enums::RenderThreadInputSignal,
        },
        task_type_enum::TaskType,
    },
};

pub struct GlobalExecuteur {
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
    executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
    ecs_thread_input_channel_sender: Sender<ECSThreadInputSignal>,
    render_thread_input_channel_sender: Sender<RenderThreadInputSignal>,
}

impl GlobalExecuteur {
    pub fn new(
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
        executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
        ecs_thread_input_channel_sender: Sender<ECSThreadInputSignal>,
        render_thread_input_channel_sender: Sender<RenderThreadInputSignal>,
    ) -> Self {
        Self {
            executeur_thread_message_bus: executeur_thread_message_bus, 
            executeur_thread_data_bus: executeur_thread_data_bus, 
            ecs_thread_input_channel_sender: ecs_thread_input_channel_sender,
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
                    TaskType::Init => {
                        self.ecs_thread_input_channel_sender.send(ECSThreadInputSignal::Init);
                        self.render_thread_input_channel_sender.send(RenderThreadInputSignal::Init);
                    },
                    TaskType::Shutdown => {
                        self.ecs_thread_input_channel_sender.send(ECSThreadInputSignal::Shutdown);
                        self.render_thread_input_channel_sender.send(RenderThreadInputSignal::Shutdown);
                    },
                    TaskType::Resize => {
                        self.ecs_thread_input_channel_sender.send(ECSThreadInputSignal::Resize);
                        self.render_thread_input_channel_sender.send(RenderThreadInputSignal::Resize);
                    },
                    TaskType::LogicCalculation => {
                        self.ecs_thread_input_channel_sender.send(ECSThreadInputSignal::LogicCalculation);
                    },
                    TaskType::PrepareRenderState => {
                        self.ecs_thread_input_channel_sender.send(ECSThreadInputSignal::PrepareRenderState);
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
