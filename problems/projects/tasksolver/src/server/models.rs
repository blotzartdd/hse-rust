pub mod requests {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct CreateTaskRequest {
        r#type: String,
        file: String,
        args: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetStatusRequest {
        id: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetTaskCountRequest;
}

pub mod responses {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct CreateTaskResponse {
        pub id: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GetStatusResponse {
        pub id: String,
        pub meta: MetaInformation,
        pub result: GetStatusResult,
    }

    #[derive(Serialize, Deserialize, Clone)]
    struct MetaInformation {
        pub created_at: String,
        pub started_at: String,
        pub finished_at: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    struct GetStatusResult {
        pub stdout: String,
        pub stderr: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetTaskCountResponse {
        pub tasks: usize,
    }
}


