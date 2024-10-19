use sane_scan;

fn main() -> anyhow::Result<()> {
    let sane = sane_scan::Sane::init_1_0()?;
    for dev in sane.get_devices()? {
        println!("{dev:?}");
    }
    Ok(())
}
