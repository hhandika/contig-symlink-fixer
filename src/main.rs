mod utils;

fn main() {
    let path = ".";
    println!("\x1b[0;44m === Generating symplink... === \x1b[0m\n");
    let count = utils::generate_symlink_all(path);
    println!("\n\x1b[0;42m DONE! \x1b[0m");
    println!("Total symlinks created: {}", count);
}
