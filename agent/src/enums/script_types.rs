use shared::db::models::Task;

#[derive(Debug)]
pub enum ScriptTypes {
    InstallScript,
    RunScript,
    DeleteScript,
}

impl ScriptTypes {
    pub fn file_name(&self) -> String{
        match self {
            ScriptTypes::InstallScript => {
                return "install.sh".to_string().clone();
            }
            ScriptTypes::RunScript => {
                return "run.sh".to_string().clone();
            }
            ScriptTypes::DeleteScript => {
                return "delete.sh".to_string().clone();
            }
        }
    }

    pub fn get_scrypt(self, task: &Task) -> Option<String>{
        match self {
            ScriptTypes::InstallScript => {
                return task.install_script.clone();
            }
            ScriptTypes::RunScript => {
                return task.run_script.clone();
            }
            ScriptTypes::DeleteScript => {
                return task.delete_script.clone();
            }
        }
    }
}