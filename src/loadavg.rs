use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::fs::File;

pub fn loadavg() -> Result<(f32, f32, f32), Box<Error>> {
    let mut line = String::new();
    try!(try!(File::open("/proc/loadavg")).read_to_string(&mut line));
    let regex = try!(Regex::new(r"^(\d+\.\d+) (\d+\.\d+) (\d+\.\d+) (\d+)/(\d+) (\d+)$"));
    if let Some(captures) = regex.captures(line.trim()) {
        if let (Some(one), Some(five), Some(fifteen)) = (captures.at(1),
                                                         captures.at(2),
                                                         captures.at(3)) {
            return Ok((try!(one.parse()), try!(five.parse()), try!(fifteen.parse())));
        }
    }
    Err(String::from("/proc/loadavg in unrecognised format").into())
}
