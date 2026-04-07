mod config;

fn main() {
    let conf = config::load("Rustakefile.toml").unwrap();
    println!("{:?}", conf);

    for (name, target) in &conf.targets {
	println!("Target '{}' : '{}'", name, target.command);
    }
}
