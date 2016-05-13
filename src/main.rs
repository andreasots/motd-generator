extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate diesel_full_text_search;
extern crate nix;
extern crate prettytable;
extern crate rand;
extern crate regex;
extern crate tempfile;
extern crate time;

use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;
use prettytable::format::{Alignment, TableFormat};
use std::io::Write;
use std::os::unix::fs::MetadataExt;

mod loadavg;
mod memory;
mod statfs;
mod quote;

fn human_readable(n: u64) -> String {
    const UNITS: [&'static str; 4] = ["B", "kiB", "MiB", "GiB"];
    let mut n = n as f64;
    for unit in &UNITS {
        if n < 1024.0 {
            return format!("{:.3} {}", n, unit);
        }
        n /= 1024.0;
    }

    format!("{:.3} TiB", n)
}

fn main() {
    match std::fs::metadata("/run/dynamic_motd") {
        Ok(ref meta) if time::get_time().sec - meta.mtime() < 60 => return,
        _ => (),
    }

    let mut output = tempfile::NamedTempFile::new_in("/run").unwrap();

    let mut host = [0; 256];
    match nix::unistd::gethostname(&mut host[..])
              .map_err(Box::<std::error::Error>::from)
              .and_then(|()| {
                  std::str::from_utf8(&host[..]).map_err(Box::<std::error::Error>::from)
              })
              .map(|host| host.trim_right_matches('\0')) {
        Ok(host) => writeln!(output, "Welcome to {}", host).unwrap(),
        Err(err) => {
            writeln!(output,
                     "Welcome to... here, I guess. (failed to get hostname: {})",
                     err)
                .unwrap()
        }
    }

    match loadavg::loadavg() {
        Ok((one, five, fifteen)) => {
            writeln!(output,
                     "  Load average:\t{:.2}, {:.2}, {:.2}",
                     one,
                     five,
                     fifteen)
                .unwrap()
        }
        Err(err) => writeln!(output, "Failed to get load averages: {}", err).unwrap(),
    }

    let mut format = TableFormat::new();
    format.column_separator(' ');
    format.padding(2, 3);

    let mut table = Table::new();
    table.set_format(format);
    table.set_titles(Row::new(vec![
        Cell::new(""),
        Cell::new_align("Total", Alignment::RIGHT),
        Cell::new_align("Free", Alignment::RIGHT),
        Cell::new_align("Available", Alignment::RIGHT),
    ]));

    match memory::get_memory_stats(&mut table) {
        Ok(()) => (),
        Err(err) => writeln!(output, "Failed to get memory stats: {}", err).unwrap(),
    }

    match statfs::disk_usage("/", &mut table) {
        Ok(()) => (),
        Err(err) => writeln!(output, "Failed to get disk stats: {}", err).unwrap(),
    }

    table.print(&mut output).unwrap();

    match quote::get_quote() {
        Ok(quote) => writeln!(output, "{}", quote).unwrap(),
        Err(err) => writeln!(output, "Failed to get quote: {}", err).unwrap(),
    }

    output.persist("/run/dynamic_motd").unwrap();
}
