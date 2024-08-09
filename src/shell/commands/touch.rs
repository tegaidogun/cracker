use std::fs::{self, OpenOptions};
use std::time::SystemTime;
use chrono::{DateTime, Local, TimeZone};
use filetime::FileTime;

use crate::shell::utils::path_utils::resolve_path;

pub fn touch(args: &[&str]) -> Result<(), String> {
    let mut no_create = false;
    let mut reference_file: Option<&str> = None;
    let mut specified_time: Option<SystemTime> = None;
    
    let mut paths = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "-c" | "--no-create" => no_create = true,
            "-r" | "--reference" => {
                if i + 1 < args.len() {
                    reference_file = Some(args[i + 1]);
                    i += 1;
                } else {
                    return Err("touch: missing argument for --reference".into());
                }
            }
            "-d" | "--date" => {
                if i + 1 < args.len() {
                    specified_time = parse_date(args[i + 1]);
                    if specified_time.is_none() {
                        return Err("touch: invalid date format".into());
                    }
                    i += 1;
                } else {
                    return Err("touch: missing argument for --date".into());
                }
            }
            "-t" => {
                if i + 1 < args.len() {
                    specified_time = parse_time(args[i + 1]);
                    if specified_time.is_none() {
                        return Err("touch: invalid time format".into());
                    }
                    i += 1;
                } else {
                    return Err("touch: missing argument for -t".into());
                }
            }
            _ => paths.push(resolve_path(args[i])),
        }
        i += 1;
    }

    if paths.is_empty() {
        return Err("touch: missing file operand".into());
    }

    let default_time = SystemTime::now();
    let new_time = match reference_file {
        Some(ref_file) => fs::metadata(resolve_path(ref_file))
            .and_then(|meta| meta.modified())
            .map_err(|e| format!("touch: failed to get modification time from reference file: {}", e))?,
        None => specified_time.unwrap_or(default_time),
    };

    let access_time = FileTime::from_system_time(new_time);
    let modification_time = FileTime::from_system_time(new_time);

    for path in paths {
        if !path.exists() && !no_create {
            OpenOptions::new().create(true).write(true).open(&path)
                .map_err(|e| format!("touch: failed to create file '{}': {}", path.display(), e))?;
        }

        if path.exists() {
            filetime::set_file_times(&path, access_time, modification_time)
                .map_err(|e| format!("touch: failed to update timestamps for '{}': {}", path.display(), e))?;
        } else if no_create {
            eprintln!("touch: cannot touch '{}': No such file or directory", path.display());
        }
    }

    Ok(())
}

fn parse_time(time_str: &str) -> Option<SystemTime> {
    if let Ok(dt) = DateTime::parse_from_str(time_str, "%Y%m%d%H%M.%S") {
        return Some(dt.with_timezone(&Local).into());
    }

    if let Ok(dt) = DateTime::parse_from_str(time_str, "%Y%m%d%H%M") {
        return Some(dt.with_timezone(&Local).into());
    }

    None
}

fn parse_date(date_str: &str) -> Option<SystemTime> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(date_str) {
        return Some(dt.with_timezone(&Local).into());
    }

    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        return Some(dt.with_timezone(&Local).into());
    }

    // Update this to use the recommended approach
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        return Some(Local.from_local_datetime(&dt).unwrap().into());
    }

    None
}
