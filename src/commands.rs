use std::process::{Command, ExitStatus};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<LsblkBlockDevice>,
}

#[derive(Serialize, Deserialize)]
struct LsblkBlockDevice {
    name: String,
    size: u64,
    fstype: Option<String>,
    children: Option<Vec<LsblkPartition>>,
}

#[derive(Serialize, Deserialize)]
struct LsblkPartition {
    name: String,
    size: u64,
}

pub fn list_disks() -> anyhow::Result<Vec<String>> {
    let output = Command::new("lsblk")
        .args(&["-b", "--json", "-o", "NAME,SIZE"])
        .output()?;

    let data = serde_json::from_str::<LsblkOutput>(
        &String::from_utf8(output.stdout)?
    )?;

    let output = data.blockdevices.into_iter()
        .map(|v| v.name)
        .collect::<Vec<String>>();

    Ok(output)
}

pub fn get_disk_size(name: &str) -> anyhow::Result<u64> {
    let output = Command::new("lsblk")
        .args(&["-b", "--json", "-o", "NAME,SIZE", &format!("/dev/{}", name)])
        .output()?;

    if !output.status.success() {
        return Err(
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Device not found"
            ).into()
        );
    }

    let data = serde_json::from_str::<LsblkOutput>(
        &String::from_utf8(output.stdout)?
    )?;

    Ok(data.blockdevices[0].size)
}

pub fn get_partitions(blockdev_name: &str) -> anyhow::Result<Option<Vec<String>>> {
    let output = Command::new("lsblk")
        .args(
            &[
                "-b", 
                "--json", 
                "-o", 
                "NAME,SIZE", 
                &format!("/dev/{}", blockdev_name)
            ]
        )
        .output()?;

    if !output.status.success() {
        return Err(
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Device not found"
            ).into()
        );
    }

    let data = serde_json::from_str::<LsblkOutput>(
        &String::from_utf8(output.stdout)?
    )?;

    match &data.blockdevices[0].children {
        Some(children) => Ok(Some(
            children.into_iter()
                .map(|v| v.name.clone())
                .collect()
        )),
        None => Ok(None),
    }
}

pub fn get_fs_type(name: &str) -> anyhow::Result<Option<String>> {
    let output = Command::new("lsblk")
        .args(
            &[
                "-b", 
                "--json", 
                "-o", 
                "NAME,SIZE,FSTYPE", 
                &format!("/dev/{}", name)
            ]
        )
        .output()?;

    if !output.status.success() {
        return Err(
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Device not found"
            ).into()
        );
    }

    let data = serde_json::from_str::<LsblkOutput>(
        &String::from_utf8(output.stdout)?
    )?;

    Ok(data.blockdevices[0].fstype.clone())
}
