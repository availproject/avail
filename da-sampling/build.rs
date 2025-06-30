use prost_build::Config;

const PROTOS: &[&str] = &["src/schema/sampling.v1.proto"];

fn main() {
	let mut config = Config::new();

	config.type_attribute("api.v1.sampling.CellCoordinate", "#[derive(Hash, Eq)]");
	config.compile_protos(PROTOS, &["src/schema"]).unwrap();
}
