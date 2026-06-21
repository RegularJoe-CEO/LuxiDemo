use clap::{Parser, Subcommand};
use luxi_demo_build::{ate, demo_guard, energy, orbital, robotics, validation};

#[derive(Parser)]
#[command(name = "luxi-tools")]
#[command(about = "Luxi demo tools - ATE, orbital, robotics, energy")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run full validation suite with SHA hashes
    Validate,
    /// Waller geodesic attention and ATE transformer demo
    Ate,
    /// Orbital mechanics solves (Kepler, Lambert, etc.)
    Orbital,
    /// Edge robotics FSD math demos
    Robotics,
    /// Physics and energy reporting
    Energy,
}

fn main() {
    demo_guard::print_demo_warning("Luxi Tools - ATE, orbital, robotics, energy");
    if !demo_guard::check_expiry() {
        std::process::exit(0);
    }

    let args = Args::parse();
    match args.command {
        Command::Validate => {
            let results = validation::run_all();
            validation::print_report(&results);
            let failed = results.iter().filter(|r| !r.passed).count();
            std::process::exit(if failed == 0 { 0 } else { 1 });
        }
        Command::Ate => ate::run_demo(),
        Command::Orbital => orbital::run_demo(),
        Command::Robotics => robotics::run_demo(),
        Command::Energy => energy::run_demo(),
    }
}