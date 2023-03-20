use std::env;

pub mod conkerfile;
pub mod docker_context;

pub struct Environment {
    pub name: String,
    pub context: String,
    configuration_file: String
}

impl Environment {
    pub fn build() -> Environment {
        let name = Self::environment_name();
        let context = format!("{}-{}", Self::application_name(), name);
        let configuration_file = format!("docker-compose.{}.yml", name);

        Environment { context, configuration_file, name }
    }

    pub fn args(&self) -> Vec<String> {
        let mut args = self.base_args();
        args.extend(self.extended_args());
        args        
    }

    fn base_args(&self) -> Vec<String> {
        vec![
            "--context".into(),
            self.context.clone(),
            "compose".into(),
            "-p".into(),
            self.context.clone(),
            "-f".into(),
            "docker-compose.yml".into(),
            "-f".into(),
            self.configuration_file.clone(),
        ]
    }

    fn extended_args(&self) -> Vec<String> {
        env::args().skip(2).collect()
    }

    fn application_name() -> String {
        env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .to_lowercase()
    }
    
    fn environment_name() -> String {
        env::args()
            .nth(1)
            .expect("expected environment as first argument")
    }
}