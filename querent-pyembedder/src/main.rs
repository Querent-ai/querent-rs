use querent_pyembedder::setup_all;

/// Should be started from the workspace dir
fn main() {
	setup_all("querent-synapse/pyembedded").expect("Failed");
	println!("Done");
}
