#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecuteurThreadTimeManagerMessage {
    LogicStart,
    FrameStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecuteurThreadTaskControllerMessage {
    ScheduleReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecuteurThreadTimeControllerMessage {
    TaskChunksReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecuteurThreadGlobalExecuteurMessage {
    JobListReady,
}
