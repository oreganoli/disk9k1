use crc32fast::Hasher;
use mime_sniffer::MimeTypeSniffer;

use crate::prelude::*;

pub(crate) enum DataError {
    TooBig { size: usize, limit: usize },
}

impl DataRepo {
    pub fn create(&self, data: &[u8], conn: &mut Conn) -> AppResult<i64> {
        eprintln!("CREATing data of length {}", data.len());
        let limit = conn
            .query("SELECT size_limit FROM instance;", &[])?
            .first()
            .map_or(0i64, |row| row.get(0)) as usize;
        let size = data.len();
        if size > limit {
            return Err(DataError::TooBig { size, limit }.into());
        }
        let mimetype = data.sniff_mime_type().unwrap_or("application/octet-stream");
        let mut hasher = Hasher::new();
        hasher.update(data);
        let hash = hasher.finalize() as i64;
        let exists = conn
            .query(
                "SELECT EXISTS (SELECT hash FROM data WHERE hash = $1);",
                &[&hash],
            )?
            .first()
            .map_or(true, |row| row.get(0));
        if !exists {
            conn.execute(
                "INSERT INTO data (hash, mimetype, data) VALUES ($1, $2, $3);",
                &[&hash, &mimetype, &data],
            )?;
            Ok(hash)
        } else {
            Ok(hash)
        }
    }
    pub fn read(&self, hash: u32, conn: &mut Conn) -> AppResult<Option<Vec<u8>>> {
        let hash = hash as i64;
        let data: Option<Vec<u8>> = conn
            .query("SELECT data FROM data WHERE hash = $1", &[&hash])?
            .first()
            .map(|row| row.get(0));
        Ok(data)
    }
}
