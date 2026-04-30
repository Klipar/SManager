use serde::Deserialize;
use crate::enums::script_types::ScriptType;

#[derive(Deserialize)]
pub struct RunTaskDTO {
    pub task_id: i64,
    pub script_type: ScriptType,
}