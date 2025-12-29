#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TaskType {
    // Logic calc
    LogicCalculation,
    // Render pipeline
    PrepareRenderState, 
    DrawRenderState,
    // Another pipeline
    Resize,
    UnknowTask,
}


impl TaskType {
    pub fn get_requirements(&self) -> Vec<TaskType> {
        match self {
            TaskType::LogicCalculation => { vec![] },
            TaskType::PrepareRenderState => { vec![] },
            TaskType::DrawRenderState => { vec![] },
            TaskType::Resize => { vec![] },
            TaskType::UnknowTask => { vec![] },
        }
    }
}
