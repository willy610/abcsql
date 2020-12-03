pub mod applic_folder;
use applic_folder::realmain::realmain;
fn main() {
    match realmain() {
        Ok(_oktxt) => eprintln!("{}", _oktxt),
        Err(txt) => eprintln!("Err {}", txt),
    }
}
