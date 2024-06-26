use macros::pipeline;
use pipe_io::core::*;
use serde::{Deserialize, Serialize};

static EXAMPLE: &str = r#"
    {
        "first_name": "kimon",
        "last_name": "vostanis",
        "thoughts": [
            {
                "time": "10:40",
                "thought": "hmm"
            },
            {
                "time": "10:41",
                "thought": "uh-huh"
            },
            {
                "time": "10:43",
                "thought": "nuh-uh"
            }
        ]
    }
"#;

#[derive(Deserialize, Debug)]
pub struct Original {
    first_name: String,
    last_name: String,
    #[serde(rename = "thoughts")]
    thoughts_and_times: Vec<Thought>,
}

#[derive(Deserialize, Debug)]
pub struct Thought {
    // time: String,
    thought: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reformatted {
    first_name: String,
    last_name: String,
    thoughts: Vec<String>,
}

pipeline! {

    // #[PostgreSQL("admin:password@localhost:8080")]
    // #[CouchDB("admin:password@localhost:8080")]
    // #[ScyllaDB("admin:password@localhost:8080")]
    Original -> Reformatted {
        
        async fn extract(&self, data: &str) -> pipe_io::Result<Original> {
            let json: Original = serde_json::from_str(data)?;
            println!("Before:\n=======\n{json:#?}\n");
            Ok(json)
        }

        async fn transform(&self, input: Original) -> pipe_io::Result<Reformatted> {
            let thoughts = input.thoughts_and_times
                .into_iter()
                .map(|row| row.thought)
                .collect();

            Ok(Reformatted {
                first_name: input.first_name,
                last_name: input.last_name,
                thoughts: thoughts,
            })
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let output_data = Pipe::<Original, Reformatted>::new()
        .extran(EXAMPLE)
        .await?;
    println!("After:\n======\n{output_data:#?}");

    Ok(())
}
