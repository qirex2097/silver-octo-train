use crate::edit::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct EditStateFile;

impl EditState for EditStateFile {}

pub fn write_file(disp: &DispField) -> std::io::Result<()> {
    let path = Path::new("hello.txt");
    let mut file = File::create(&path)?;

    let json_data = serde_json::to_string(&disp.blocks).unwrap();
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

pub fn read_file(disp: &mut DispField) -> std::io::Result<()> {
    let path = Path::new("hello.txt");
    let mut file = File::open(&path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    disp.blocks = serde_json::from_str(&s)?;
    disp.rebuild_block();

    Ok(())
}
