#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlThreadInputSignal {
    LogicStart,
    FrameStart,
    Init,
    Shutdown,
}
