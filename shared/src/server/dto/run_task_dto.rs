use serde::{Deserialize, Serialize};
use crate::enums::script_types::ScriptType;

#[derive(Deserialize,Serialize)]
pub struct RunTaskDTO {
    pub task_id: i64,
    pub script_type: ScriptType,
}