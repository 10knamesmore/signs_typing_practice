mod app;
mod signs;
mod terminal_ops;

fn main() {
    app::run(signs::SymblesChoice::OnlyNumbers);
}
