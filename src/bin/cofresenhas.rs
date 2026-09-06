fn main() {
    if let Err(err) = cofresenhas_rs::cli::run() {
        eprintln!("Erro: {err}");
        std::process::exit(1);
    }
}
