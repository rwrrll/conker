use conker::Environment;
use conker::conkerfile::Conkerfile;
use conker::docker_context::DockerContext;
use std::process::exit;
use std::process::Command;

fn main() {
    let environment = Environment::build();

    if environment.name == "--init" {
        Conkerfile::write_fresh();
        println!("🌰 created Conkerfile");
        exit(0);
    }

    if environment.name == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let host = Conkerfile::host_for(&environment.name);
    DockerContext::apply_endpoint(&environment.context, &host);

    Command::new("docker")
        .args(environment.args())
        .status()
        .expect("command failed");
}
