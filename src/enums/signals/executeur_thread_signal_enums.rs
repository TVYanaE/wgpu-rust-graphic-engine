#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecuteurThreadInputSignal {
    ScheduleReady,
    LogicStart,
    FrameStart,
}
