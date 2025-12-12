#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Task {
    RenderFrame,
    PhysicsCalculation,
}
