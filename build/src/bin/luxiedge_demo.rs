use clap::{Parser, Subcommand};
use luxi_demo_build::demo_guard;
use luxi_demo_build::eval;
use luxi_demo_build::server;
use luxi_demo_build::validation;

#[derive(Parser)]
#[command(name = "luxiedge-demo")]
#[command(about = "LuxiEdge Demo - extended operator set (demo only)")]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(short, long, default_value = "9090")]
    port: u16,
}

#[derive(Subcommand)]
enum Command {
    /// Run the validation test suite
    Validate,
    /// List supported expressions
    List,
}

#[tokio::main]
async fn main() {
    demo_guard::print_demo_warning("LuxiEdge Demo - 30 operators (NOT the full private build)");
    if !demo_guard::check_expiry() {
        std::process::exit(0);
    }

    let args = Args::parse();
    match args.command {
        Some(Command::Validate) => {
            let results = validation::run_all();
            validation::print_report(&results);
            let failed = results.iter().filter(|r| !r.passed).count();
            std::process::exit(if failed == 0 { 0 } else { 1 });
        }
        Some(Command::List) => {
            println!("Demo operators (full private build has 30+):");
            for e in eval::demo::supported_exprs() {
                println!("  {}", e);
            }
        }
        None => {
            let ops = eval::demo::supported_exprs().len();
            let handler = server::make_eval_handler(|expr, values, use_f64| {
                eval::demo::evaluate(expr, values, use_f64)
            });
            server::run_server(
                args.port,
                "LUXIEDGE DEMO",
                "30 operators demo",
                ops,
                handler,
            )
            .await;
        }
    }
}