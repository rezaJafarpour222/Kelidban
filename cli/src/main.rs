use crate::app::App;

pub mod app;

fn main() {
    // let path = String::from("test.kdb");
    // let password = b"password";
    // let mut app = app::App::new(Vault::new());
    // app.add_entry(
    //     "GitHub",
    //     "user123",
    //     "my-secret-password",
    //     "https://github.com",
    //     "My GitHub account",
    // );
    // app.save(path, password).unwrap();
    //------------Loading---------------------

    // let app = App::load(path, password).unwrap();
    // let entries = app.entries();
    // println!("entries:[{}]", entries.len());
    // println!("UUID: {:}", entries[0].uuid().unwrap());
    // println!(
    //     "Title: {:}",
    //     String::from_utf8_lossy(entries[0].title().unwrap_or_default())
    // );
    //
    // println!(
    //     "Username: {:}",
    //     String::from_utf8_lossy(entries[0].username().unwrap_or_default())
    // );
    //
    // println!(
    //     "Password: {:}",
    //     String::from_utf8_lossy(entries[0].password().unwrap_or_default())
    // );
    //
    // println!(
    //     "Notes: {:}",
    //     String::from_utf8_lossy(entries[0].notes().unwrap_or_default())
    // );
    //
    // println!(
    //     "Url: {:}",
    //     String::from_utf8_lossy(entries[0].url().unwrap_or_default())
    // );
}
