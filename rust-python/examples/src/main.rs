// mod hello_embed;
mod freeze;

fn main() {
    // let result = hello_embed::main();
    let result = freeze::main();
    if let Err(_e) = result {
        eprintln!("hata");
    }
}
