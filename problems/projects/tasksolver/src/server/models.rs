pub mod requests {
    use serde::{Deserialize, Serialize};

    /// Struct of create task request (POST)
    #[derive(Serialize, Deserialize, Clone)]
    pub struct CreateTaskRequest {
        // Type of file (python/bin)
        pub r#type: String,
        // Python script or base64 encoded binary file
        pub file: String,
        // Arguments of executable
        pub args: String,
    }

    /// Struct of get status request (GET)
    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetStatusRequest {
        /// UUID of task
        pub id: String,
    }

    /// Struct of get task count request (GET)
    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetTaskCountRequest;
}

pub mod responses {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    /// Struct of create task response
    #[derive(Serialize, Deserialize)]
    pub struct CreateTaskResponse {
        /// UUID of task
        pub id: String,
    }

    /// Struct of get status response
    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetStatusResponse {
        /// Task status (WAIT/RUNNING/SUCCESS/ERROR)
        pub status: String,
        /// Meta information (created_at, started_at, finished_at)
        pub meta: MetaInformation,
        /// Execution result (stdout, stderr)
        pub result: GetStatusResult,
    }

    impl GetStatusResponse {
        /// Creates new status response to initialize task status
        pub fn new() -> GetStatusResponse {
            let meta = MetaInformation {
                created_at: Utc::now().to_string(),
                started_at: None,
                finished_at: None,
            };

            let result = GetStatusResult {
                stdout: "".to_string(),
                stderr: None,
            };

            GetStatusResponse {
                status: "WAIT".to_string(),
                meta,
                result,
            }
        }
    }

    /// Struct of meta information for task
    #[derive(Serialize, Deserialize, Clone)]
    pub struct MetaInformation {
        /// UTC time of create task
        pub created_at: String,
        /// UTC time of starting task
        #[serde(skip_serializing_if = "Option::is_none")]
        pub started_at: Option<String>,
        /// UTC time of finishing task
        #[serde(skip_serializing_if = "Option::is_none")]
        pub finished_at: Option<String>,
    }

    /// Struct of get status request result
    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetStatusResult {
        /// Stdout of executable file
        pub stdout: String,
        /// Stderr of executable file
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stderr: Option<String>,
    }

    /// Struct of get task count response
    #[derive(Serialize, Deserialize)]
    pub struct GetTaskCountResponse {
        /// Amount of tasks in queue
        pub tasks: usize,
    }
}
