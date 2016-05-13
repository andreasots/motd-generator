use nix::NixPath;
use nix::sys::statfs::statfs;
use nix::sys::statfs::vfs::Statfs;
use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;
use prettytable::format::Alignment;
use std::error::Error;
use std::fmt::Display;
use super::human_readable;

pub fn disk_usage<P: ?Sized + NixPath + Display>(path: &P,
                                                 table: &mut Table)
                                                 -> Result<(), Box<Error>> {
    let mut stat = Statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: 0,
        f_namelen: 0,
        f_frsize: 0,
        f_spare: [0; 5],
    };

    try!(statfs(path, &mut stat));

    table.add_row(Row::new(vec![
        Cell::new(&format!("{}", path)),
        Cell::new_align(&human_readable(stat.f_blocks * stat.f_bsize as u64), Alignment::RIGHT),
        Cell::new_align(&human_readable(stat.f_bfree * stat.f_bsize as u64), Alignment::RIGHT),
        Cell::new_align(&human_readable(stat.f_bavail * stat.f_bsize as u64), Alignment::RIGHT),
    ]));

    Ok(())
}
