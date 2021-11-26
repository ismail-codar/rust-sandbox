mod hello_embed;

fn main() {
    let result = hello_embed::main();
    if let Err(_e) = result {
        eprintln!("hata");
    }
}
