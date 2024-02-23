slint::include_modules!();

fn main() {
    let main_window: MainWindow = MainWindow::new().unwrap();

    main_window.run().unwrap();
}
