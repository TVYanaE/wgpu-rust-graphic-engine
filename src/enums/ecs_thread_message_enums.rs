#[derive(Debug, Clone, Copy)]
pub enum ECSThreadWorldManagerMessage {
    Init,
    Shutdown,
    LogicCalculation,
    PrepareRenderState,
    Resize,
}
