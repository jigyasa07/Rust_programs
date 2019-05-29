mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("inside clarinet");
            super::breathe_in();
        }
    }

pub fn breathe_in() {
   println!("inside breathe_in");
}

}


fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}

