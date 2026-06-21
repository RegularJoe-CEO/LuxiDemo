use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEMO_DAYS: u64 = 10;

fn build_epoch() -> u64 {
    option_env!("LUXI_BUILD_EPOCH")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1740000000)
}

pub fn print_demo_warning(label: &str) {
    println!();
    println!("  ============================================================");
    println!("  DEMO BUILD ONLY");
    println!("  {}", label);
    println!("  This binary is for evaluation. Not for production use.");
    println!("  Full builds with all operators are available under license.");
    println!("  Contact: e@ewaller.com");
    println!("  ============================================================");
    println!();
}

pub fn check_expiry() -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs();
    let built = build_epoch();
    let limit = built + DEMO_DAYS * 24 * 60 * 60;
    if now > limit {
        let days_ago = (now - limit) / 86400;
        println!();
        println!("  This demo build has expired.");
        println!("  It was valid for {} days from the build date.", DEMO_DAYS);
        println!("  Expired about {} day(s) ago.", days_ago.max(1));
        println!("  Download a fresh build from:");
        println!("  https://github.com/RegularJoe-CEO/LuxiDemo/releases");
        println!("  Questions: e@ewaller.com");
        println!();
        return false;
    }
    let left = (limit - now) / 86400;
    println!("  Demo expires in {} day(s).", left);
    true
}