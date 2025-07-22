use bson::Document;
use std::fs;

#[allow(dead_code)]
pub fn write(path: &std::path::Path, database: Document) -> Result<(), Box<dyn std::error::Error>> {
    let database_bytes: Vec<u8> = bson::to_vec(&database)?;
    fs::write(path, database_bytes)?;
    Ok(())
}

#[allow(dead_code)]
pub fn read(path: &std::path::Path) -> Result<Document, Box<dyn std::error::Error>> {
    let database_bytes: Vec<u8> = fs::read(path)?;
    let database: Document = Document::from_reader(&mut &database_bytes[..])?;
    Ok(database)
}
