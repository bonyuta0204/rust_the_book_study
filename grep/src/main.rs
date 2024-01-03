use std::env;

use grep::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(env::args())?;

    grep::run(config)
}
