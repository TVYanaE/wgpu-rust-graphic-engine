#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    InitPhase,
    ShutdownPhase,
    ExternalEventsPhase,
    UpdatePhase,
    RenderPhase,
    Idle,
}
