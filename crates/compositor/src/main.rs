mod runtime;
mod backend;
mod seat;
mod output;

fn main() {
    println!("Kevyt compositor scaffold (Smithay target)");
    runtime::run();
}
