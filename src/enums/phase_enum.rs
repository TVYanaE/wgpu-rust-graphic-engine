#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    InitPhase,
    ShutdownPhase,
    ExternalEventsPhase,
    UpdatePhase,
    RenderPhase,
    Idle,
}
