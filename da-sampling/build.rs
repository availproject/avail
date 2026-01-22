use prost_build::Config;

const PROTOS: &[&str] = &["src/schema/sampling.v1.proto"];

fn main() {
	println!("cargo:rerun-if-changed=src/schema/sampling.v1.proto");

	let mut config = Config::new();
	// config.type_attribute(".", "#[derive(Clone, PartialEq, Eq)]");
	config
		.compile_protos(PROTOS, &["src/schema"])
		.expect("Failed to compile DA sampling protos");
}
