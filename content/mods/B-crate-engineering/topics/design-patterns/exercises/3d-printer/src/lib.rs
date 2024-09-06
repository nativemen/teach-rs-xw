use std::marker::PhantomData;

use rand::Rng;

pub struct Printer3D<S> {
    _marker: PhantomData<S>,
}

impl<CS> Printer3D<CS> {
    fn into_state<NS>(self) -> Printer3D<NS> {
        Printer3D {
            _marker: PhantomData,
        }
    }
}

/* States */

/// The 3D printer encountered an error and needs resetting
pub enum ErrorState {}
/// The 3D printer is waiting for a job
pub enum IdleState {}
/// The 3D printer is currently printing
pub enum PrintingState {}
/// The 3D printed product is ready
pub enum ProductReadyState {}

/// Check if we're out of filament
fn out_of_filament() -> bool {
    let rand: usize = rand::thread_rng().gen_range(0..100);
    rand > 95
}

impl Printer3D<IdleState> {
    pub fn new() -> Self {
        println!("Create Printer3D");
        Self {
            _marker: PhantomData,
        }
    }

    pub fn start_print(self) -> Printer3D<PrintingState> {
        println!("Start the print job...");
        self.into_state()
    }
}

impl Default for Printer3D<IdleState> {
    fn default() -> Self {
        Self::new()
    }
}

impl Printer3D<PrintingState> {
    pub fn check_filament(self) -> Result<Self, Printer3D<ErrorState>> {
        if out_of_filament() {
            println!("Out of filament!");
            Err(self.into_state())
        } else {
            Ok(self)
        }
    }

    pub fn finish_print(self) -> Printer3D<ProductReadyState> {
        println!("Finish the print job...");
        self.into_state()
    }
}

impl Printer3D<ErrorState> {
    pub fn reset(self) -> Printer3D<IdleState> {
        println!("Reset the printer!");
        self.into_state()
    }
}

impl Printer3D<ProductReadyState> {
    pub fn retrieve_product(self) -> Printer3D<IdleState> {
        println!("Retrieve product!");
        self.into_state()
    }
}
