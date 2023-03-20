use std::fs;
use std::process::exit;

#[derive(Clone)]
pub struct Entry(pub String, pub String);

#[derive(Clone)]
pub struct Conkerfile {
    entries: Vec<Entry>,
}

impl Conkerfile {
    pub fn write_fresh() {
        fs::write(
            "Conkerfile",
            "# environment_name docker_engine_url\n# e.g.\n# staging tcp://1.2.3.4:2375\n# production ssh://user@hostname",
        )
        .expect("🌰 Could not create Conkerfile");
    }

    pub fn load() -> Conkerfile {
        let entries: Vec<Entry> = Self::read()
            .lines()
            .filter(|line| !line.starts_with('#'))
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 2 {
                    eprintln!("🌰 Invalid Conkerfile entry: {}", line);
                    exit(1);
                }
                Entry(parts[0].to_string(), parts[1].to_string())
            })
            .collect();

        Conkerfile { entries }
    }

    pub fn host_for(environment: &str) -> String {
        let conkerfile = Self::load();
        conkerfile.find_environment(environment).1
    }

    fn read() -> String {
        match fs::read_to_string("Conkerfile") {
            Ok(c) => c,
            Err(_) => {
                eprintln!("🌰 No Conkerfile found 😢 use `conker init` to create one");
                exit(1)
            }
        }
    }

    fn find_environment(&self, environment: &str) -> Entry {
        match self.entries.iter().find(|c| c.0 == environment) {
            Some(c) => c.clone(),
            None => {
                eprintln!("🌰 Conkerfile has no environment `{}`", environment);
                exit(1);
            }
        }
    }
}
