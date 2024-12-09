pub mod requests {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct CreateTaskRequest {
        pub r#type: String,
        pub file: String,
        pub args: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetStatusRequest {
        pub id: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetTaskCountRequest;
}

pub mod responses {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct CreateTaskResponse {
        pub id: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetStatusResponse {
        pub status: String,
        pub meta: MetaInformation,
        pub result: GetStatusResult,
    }

    impl GetStatusResponse {
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

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MetaInformation {
        pub created_at: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub started_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub finished_at: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetStatusResult {
        pub stdout: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stderr: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetTaskCountResponse {
        pub tasks: usize,
    }
}
