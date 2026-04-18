use shared::db::models::Task;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "script_type", rename_all = "lowercase")]
pub enum ScriptType {
    Install,
    Run,
    Delete,
}

impl ScriptType {
        pub fn file_name(&self) -> &'static str {
        match self {
            ScriptType::Install => "install.sh",
            ScriptType::Run => "run.sh",
            ScriptType::Delete => "delete.sh",
        }
    }

    pub fn get_script<'a>(&self, task: &'a Task) -> Option<&'a str> {
        match self {
            ScriptType::Install => task.install_script.as_deref(),
            ScriptType::Run => task.run_script.as_deref(),
            ScriptType::Delete => task.delete_script.as_deref(),
        }
    }
}