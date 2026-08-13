#[path = "showcase/mod.rs"]
mod showcase;

fn main() {
    let component = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "overview".to_owned());
    showcase::run_native(&component);
}
