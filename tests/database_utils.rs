#[cfg(test)]
use bson::{Document, doc};
use std::fs;
use std::path::Path;

mod database {
    use super::*;

    #[test]
    fn write_utils() -> Result<(), Box<dyn std::error::Error>> {
        //? Write temp file
        let temp = tempfile::NamedTempFile::new()?;
        let path = temp.path();
        let data: Document = doc! {
            "name": "alice",
            "age": 30
        };

        //? Write to database
        eravodb::database::utils::write(path, data.clone())?;

        //? Is temp file exist
        assert!(Path::new(path).exists(), "File not exists");

        //? Is File Correct
        let bytes = fs::read(path)?;
        let read_doc: Document = bson::from_slice(&bytes)?;

        assert_eq!(read_doc, data, "File not correct");

        Ok(())
    }

    #[test]
    fn read_utils() -> Result<(), Box<dyn std::error::Error>> {
        //? Write temp file
        let temp = tempfile::NamedTempFile::new()?;
        let path = temp.path();
        let data: Document = doc! {
            "name": "alice",
            "age": 30
        };
        let database_bytes: Vec<u8> = bson::to_vec(&data)?;
        fs::write(path, database_bytes)?;

        //? Is File Correct
        let read_database = eravodb::database::utils::read(path)?;
        assert_eq!(read_database, data, "File not correct");

        Ok(())
    }
}
