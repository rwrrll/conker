use serde_derive::Deserialize;
use std::process::exit;
use std::process::Command;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct DockerContext {
    Name: String,
}

impl DockerContext {
    pub fn apply_endpoint(context_name: &str, host: &str) {
        let contexts = Self::contexts();
        let context = contexts.iter().find(|c| c.Name == context_name);
        let operation = match context {
            Some(_) => "update",
            None => "create",
        };
        let host: &str = &format!("host={}", host);
        let args = ["context", operation, &context_name, "--docker", host];

        let output = Command::new("docker")
            .args(args)
            .output()
            .expect("failed to execute docker context operation");

        if !output.status.success() {
            eprintln!("🌰 docker context {} failed: {}", operation, output.status);
            eprintln!("🐳 {}", String::from_utf8(output.stderr).unwrap());
            exit(1);
        };
    }

    fn contexts() -> Vec<DockerContext> {
        let output = Command::new("docker")
            .args(["context", "ls", "--format", "json"])
            .output()
            .expect("could not list contexts");
        let json = String::from_utf8(output.stdout).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}
