#[derive(Debug, Clone, Copy)]
pub enum LogicThreadInputSignal {
    Init,
    Shutdown,
    Resize,
    LogicCalculation,
    PrepareRenderState,
}
