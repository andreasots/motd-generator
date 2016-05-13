use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;
use prettytable::format::Alignment;
use regex::Regex;
use super::human_readable;

pub fn get_memory_stats(table: &mut Table) -> Result<(), Box<Error>> {
    let regex = try!(Regex::new(r"^([^:]+):\s*(\d+)(?: kB)?$"));

    let mut meminfo = HashMap::new();

    for line in BufReader::new(try!(File::open("/proc/meminfo"))).lines() {
        let line = try!(line);
        if let Some(captures) = regex.captures(&line) {
            match (captures.at(1), captures.at(2)) {
                (Some(key), Some(value)) => {
                    meminfo.insert(String::from(key), try!(value.parse::<u64>()))
                }
                (_, _) => continue,
            };
        }
    }

    table.add_row(Row::new(vec![
        Cell::new("Mem:"),
        Cell::new_align(&human_readable(meminfo["MemTotal"]*1024), Alignment::RIGHT),
        Cell::new_align(&human_readable(meminfo["MemFree"]*1024), Alignment::RIGHT),
        Cell::new_align(&human_readable(meminfo["MemAvailable"]*1024), Alignment::RIGHT),
    ]));
    table.add_row(Row::new(vec![Cell::new("Swap:"),
                                Cell::new_align(&human_readable(meminfo["SwapTotal"] * 1024),
                                                Alignment::RIGHT),
                                Cell::new_align(&human_readable(meminfo["SwapFree"] * 1024),
                                                Alignment::RIGHT),
                                Cell::new("")]));

    Ok(())
}
