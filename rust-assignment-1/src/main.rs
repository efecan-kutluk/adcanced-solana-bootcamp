pub mod pubs;
use pubs::dummy_pubs;

fn main() {
    let publications = dummy_pubs();

    for publication in &publications {
        publication.view();
    }
}
