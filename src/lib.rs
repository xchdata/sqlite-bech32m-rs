use bech32::{self, FromBase32, ToBase32, Variant};
use rusqlite::functions::{Context, FunctionFlags};
use rusqlite::types::{ToSqlOutput, Value};

#[cfg(feature = "extension")]
mod ext;

fn ah(e: anyhow::Error) -> rusqlite::Error {
    rusqlite::Error::UserFunctionError(format!("{:?}", e).into())
}

fn setup(db: &rusqlite::Connection) -> anyhow::Result<()> {
    create_functions(&db)
}

fn create_functions(db: &rusqlite::Connection) -> anyhow::Result<()> {
    let flags = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_INNOCUOUS |
                FunctionFlags::SQLITE_DETERMINISTIC;
    db.create_scalar_function("bech32m_encode",
                                2,
                                flags,
                                |ctx| bech32m_encode_fn(ctx).map_err(ah))?;
    db.create_scalar_function("bech32m_decode",
                                1,
                                flags,
                                |ctx| bech32m_decode_fn(ctx).map_err(ah))?;
    Ok(())
}

fn bech32m_encode_fn<'a>(ctx: &Context) -> anyhow::Result<ToSqlOutput<'a>> {
    let hrp = ctx.get::<String>(0)?;
    let data = ctx.get::<Vec<u8>>(1)?;
    let encoded = bech32::encode(&hrp, data.to_base32(), Variant::Bech32m)?;
    Ok(ToSqlOutput::Owned(Value::Text(encoded)))
}

fn bech32m_decode_fn<'a>(ctx: &Context) -> anyhow::Result<ToSqlOutput<'a>> {
    let encoded = ctx.get::<String>(0)?;
    let (_hrp, data, _variant) = bech32::decode(&encoded)?;
    Ok(ToSqlOutput::Owned(Value::Blob(Vec::<u8>::from_base32(&data)?)))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use anyhow::Context;
    pub use pretty_assertions::{assert_eq, assert_ne};

    use rusqlite::Connection;

    pub fn open_db() -> anyhow::Result<Connection> {
        let db = Connection::open_in_memory().context("open in-memory db")?;
        setup(&db).context("setup db")?;
        Ok(db)
    }

    #[test]
    fn sanity() -> anyhow::Result<()> {
        open_db()?;
        Ok(())
    }

    #[test]
    fn bech32m_encode_works() -> anyhow::Result<()> {
        let db = open_db()?;
        assert_eq!("xch1jlgazv".to_string(),
                   db.query_row("select bech32m_encode('xch', x'')",
                                [],
                                |r| r.get::<usize, String>(0))?);
        assert_eq!("xch1etlqusgk05".to_string(),
                   db.query_row("select bech32m_encode('xch', x'cafe')",
                                [],
                                |r| r.get::<usize, String>(0))?);
        assert_eq!("xch17nmv5574vggcdxchqh8zjunt44ax05cwhcqz5e29pvf6mwc95e5s27yfa4".to_string(),
                   db.query_row("select bech32m_encode('xch', x'f4f6ca53d56211869b1705ce29726bad7a67d30ebe002a65450b13adbb05a669')",
                                [],
                                |r| r.get::<usize, String>(0))?);
        Ok(())
    }

    #[test]
    fn bech32m_decode_works() -> anyhow::Result<()> {
        let db = open_db()?;
        assert_eq!("".to_string(),
                   db.query_row("select hex(bech32m_decode('xch1jlgazv'))",
                                [],
                                |r| r.get::<usize, String>(0))?);
        assert_eq!("CAFE".to_string(),
                   db.query_row("select hex(bech32m_decode('xch1etlqusgk05'))",
                                [],
                                |r| r.get::<usize, String>(0))?);
        assert_eq!("F4F6CA53D56211869B1705CE29726BAD7A67D30EBE002A65450B13ADBB05A669".to_string(),
                   db.query_row("select hex(bech32m_decode('xch17nmv5574vggcdxchqh8zjunt44ax05cwhcqz5e29pvf6mwc95e5s27yfa4'))",
                                [],
                                |r| r.get::<usize, String>(0))?);
        Ok(())
    }
}
