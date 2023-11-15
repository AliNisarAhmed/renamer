fn main() {
    if let Err(e) = renamer::get_args().and_then(renamer::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
