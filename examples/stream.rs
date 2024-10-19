use std::{env, ffi::CString};

use sane_scan::{self, DeviceHandle, DeviceOption, DeviceOptionValue, OptionInfo};

fn get_option_by_name<'o>(name: &str, options: &'o [DeviceOption]) -> Option<&'o DeviceOption> {
    let name = CString::new(name).expect("invalid CString");
    for o in options {
        if o.name == name {
            return Some(o);
        }
    }
    None
}

trait SaneExt {
    fn set_opt_string(
        &self,
        opt_name: &str,
        opt_value: &str,
    ) -> Result<OptionInfo, sane_scan::Error>;
    fn set_opt_fixed(&self, opt_name: &str, opt_value: i32)
        -> Result<OptionInfo, sane_scan::Error>;
    fn print_options(&self) -> Result<(), sane_scan::Error>;
    fn print_parameters(&self) -> Result<(), sane_scan::Error>;
}

impl SaneExt for DeviceHandle {
    fn set_opt_string(
        &self,
        opt_name: &str,
        opt_value: &str,
    ) -> Result<OptionInfo, sane_scan::Error> {
        let options = self.get_options()?;
        let opt = get_option_by_name(opt_name, &options).expect("missing option");
        let value = DeviceOptionValue::String(CString::new(opt_value).expect("bad CString"));
        let opt_info = self.set_option(&opt, value)?;
        if opt_info.intersects(OptionInfo::INFO_RELOAD_PARAMS) {
            _ = self.get_parameters()?;
        }
        Ok(opt_info)
    }
    fn set_opt_fixed(
        &self,
        opt_name: &str,
        opt_value: i32,
    ) -> Result<OptionInfo, sane_scan::Error> {
        let options = self.get_options()?;
        let opt = get_option_by_name(opt_name, &options).expect("missing option");
        let value = DeviceOptionValue::Fixed(opt_value);
        let opt_info = self.set_option(&opt, value)?;
        if opt_info.intersects(OptionInfo::INFO_RELOAD_PARAMS) {
            _ = self.get_parameters()?;
        }
        Ok(opt_info)
    }
    fn print_options(&self) -> Result<(), sane_scan::Error> {
        let options = self.get_options()?;
        println!("Options:");
        for o in &options {
            println!("\t{o:?}");
        }
        Ok(())
    }
    fn print_parameters(&self) -> Result<(), sane_scan::Error> {
        let parameters = self.get_parameters()?;
        println!("parameters:\n{parameters:#?}");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        env::set_var("SANE_DEBUG_DLL", "255");
        env::set_var("SANE_DEBUG_SANEI_CONFIG", "255");
        env::set_var("SANE_DEBUG_NET", "255");
    }
    let sane = sane_scan::Sane::init_1_0()?;
    let desired_device = "brother5:net1;dev0";
    for dev in sane.get_devices()? {
        println!("{dev:?}");
        if dev.name.to_str()? == desired_device {
            println!("Opening {desired_device}");
            println!("\t{:?}", dev.name);
            println!("\t{:?}", dev.vendor);
            println!("\t{:?}", dev.model);
            println!("\t{:?}", dev.type_);
            let mut dh = dev.open()?;
            dh.print_options()?;
            dh.print_parameters()?;

            println!(
                "Setting source: {:?}",
                dh.set_opt_string("source", "FlatBed")?
            );
            /*
                        println!(
                            "Setting mode: {:?}",
                            dh.set_opt_string("mode", "24bit Color[Fast]")?
                        );

                        dh.print_options()?;


                        println!("Setting tl-x: {:?}", dh.set_opt_fixed("tl-x", 0)?);
                        println!("Setting tl-y: {:?}", dh.set_opt_fixed("tl-y", 0)?);
                        println!("Setting br-x: {:?}", dh.set_opt_fixed("br-x", 10)?);
                        println!("Setting br-y: {:?}", dh.set_opt_fixed("br-y", 10)?);

                        println!(
                            "Setting resolution: {:?}",
                            dh.set_opt_fixed("resolution", 100)?
                        );

                        dh.print_options()?;
            */
            println!("Starting scan");
            println!("{:#?}", dh.start_scan()?);
            let mut buf = [0; 1024 * 1024];
            println!("Starting read");
            while let Some(n) = dh.read(&mut buf[..])? {
                println!("read {n} bytes");
            }
        }
    }
    println!("exiting");
    Ok(())
}
