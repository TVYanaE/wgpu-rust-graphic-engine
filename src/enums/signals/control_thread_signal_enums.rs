#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlThreadInputSignal {
    EngineSignal(ControlThreadInputEngineSignal),
    GameSignal(ControlThreadInputGameSignal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlThreadInputEngineSignal {
    LogicStart,
    FrameStart,
    Init,
    Shutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlThreadInputGameSignal {
    GameplayStarted
}
