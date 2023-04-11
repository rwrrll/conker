use std::process::exit;
use std::process::Command;

pub struct DockerContext {
}

impl DockerContext {
    pub fn apply_endpoint(context_name: &str, host: &str) {
        let operation = match Self::exists(&context_name) {
            true => "update",
            false => "create",
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

    fn exists(context_name: &str) -> bool {
        let binding = Self::ls();
        let contexts: Vec<&str> = binding
            .split("\n")
            .filter(|c| c.len() > 4)
            .map(|c| &c[2..c.len() - 2])
            .collect();
        contexts.contains(&context_name)
    }

    fn ls() -> String {
        let output = Command::new("docker")
            .args(["context", "ls", "--format", "'{{ json .Name }}'"])
            .output()
            .expect("could not list contexts");
        String::from_utf8(output.stdout).unwrap()
    }
}
