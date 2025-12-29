use winit::{
    event::{WindowEvent},
    dpi::PhysicalSize,
};

#[derive(Debug, Clone)]
pub enum WinitEvent {
    WindowEvent(WindowEvent),
}

impl From<WindowEvent> for WinitEvent {
    fn from(value: WindowEvent) -> Self {
        Self::WindowEvent(value)
    }
}

#[derive(Debug, Clone)]
pub enum IOEvent {
    WinitEvent(WinitEvent), 
}

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    LogicCalculation,
    PrepareRenderState, 
    DrawRenderState, 
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    LogicCalculation,
    PrepareRenderState, 
    DrawRenderState,
    Resize(PhysicalSize<u32>),
    UnknownEvent,
}

impl From<GameEvent> for Event {
    fn from(value: GameEvent) -> Self {
        match value {
            GameEvent::LogicCalculation => {
                Self::LogicCalculation
            },
            GameEvent::PrepareRenderState => {
                Self::PrepareRenderState
            },
            GameEvent::DrawRenderState => {
                Self::DrawRenderState
            },
        }
    }
}

impl From<IOEvent> for Event {
    fn from(value: IOEvent) -> Self {
        match value {
            IOEvent::WinitEvent(winit_event) => {
                match winit_event {
                    WinitEvent::WindowEvent(window_event) => {
                        match window_event {
                            WindowEvent::Resized(physical_size) => {
                                Self::Resize(physical_size)
                            },
                            _ => {
                                Self::UnknownEvent
                            }
                        }
                    }
                }
            }
        }
    }
}
