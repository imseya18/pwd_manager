use rusqlite::Error as RusqliteError;
use uuid::Uuid;

pub fn convert_uid_from_db(uid_str: String) -> rusqlite::Result<Uuid> {
    let uid = Uuid::parse_str(&uid_str)
        .map_err(|e| RusqliteError::UserFunctionError(Box::new(e)))?;
    Ok(uid)
}
