use print3d::*;

fn main() {
    let mut printer = Printer3D::new();

    let mut count = 0;
    while count < 100 {
        let printing_printer = printer.start_print();
        printer = match printing_printer.check_filament() {
            Err(printer) => printer.reset(),
            Ok(printer) => {
                let printer = printer.finish_print();
                printer.retrieve_product()
            }
        };

        count += 1;
    }
}
