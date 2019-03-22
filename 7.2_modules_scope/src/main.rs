// If performance_group's use of the instrument isn't declared as pub, we need this
// in order to use the instrument from main.
use sound::instrument;
mod animal;

fn main() {
    performance_group::clarinet_trio();
    println!("SEPARATION");
    // performance_group::instrument::clarinet();
    instrument::clarinet();

    animal::pet::dog();
}

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("CLARINET");
        }
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}