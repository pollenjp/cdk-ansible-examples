use my_app::run;

pub fn main() -> std::result::Result<(), i32> {
    if let Err(e) = run() {
        eprintln!("Error: {e:?}");
        return Err(1);
    }
    Ok(())
}
