use std::collections::HashMap;
use sysinfo::{DiskExt, System, SystemExt};

pub fn get_disks(disks: &mut HashMap<String, u64>) {
    let s = System::new_all();
    for disk in s.disks() {
        disks.insert(
            disk.mount_point().display().to_string(),
            disk.available_space(),
        );
    }
}
