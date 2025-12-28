#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    ExternalEventsPhase,
    UpdatePhase,
    RenderPhase,
    Idle,
}
