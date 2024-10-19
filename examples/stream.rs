use sane_scan;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sane = sane_scan::Sane::init_1_0()?;
    for dev in sane.get_devices()? {
        println!("{dev:?}");
    }
    Ok(())
}
