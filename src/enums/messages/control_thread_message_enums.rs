#[derive(Debug, Clone, Copy)]
pub enum ControlThreadRequestManagerMessage {
    FrameStart,
    LogicStart,
    InitSignal,
    ShutdownSignal,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlThreadSceneManagerMessage {
    GameplayStarted, 
}
