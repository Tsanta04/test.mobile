use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = crate::schema::sql_types::Alerttype)]
pub enum AlertType {
    Health,
    Production,
    Rentability,
    Humidity,
    Fertility,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = crate::schema::sql_types::Leveltype)]
pub enum LevelType {
    Low,
    Medium,
    High,
    Urgent,
}

impl ToSql<crate::schema::sql_types::Alerttype, Pg> for AlertType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            AlertType::Health => out.write_all(b"Health")?,
            AlertType::Production => out.write_all(b"Production")?,
            AlertType::Rentability => out.write_all(b"Rentability")?,
            AlertType::Humidity => out.write_all(b"Humidity")?,
            AlertType::Fertility => out.write_all(b"Fertility")?,
            AlertType::Other => out.write_all(b"Other")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Alerttype, Pg> for AlertType {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Health" => Ok(AlertType::Health),
            b"Production" => Ok(AlertType::Production),
            b"Rentability" => Ok(AlertType::Rentability),
            b"Humidity" => Ok(AlertType::Humidity),
            b"Fertility" => Ok(AlertType::Fertility),
            b"Other" => Ok(AlertType::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<crate::schema::sql_types::Leveltype, Pg> for LevelType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            LevelType::Low => out.write_all(b"Low")?,
            LevelType::Medium => out.write_all(b"Medium")?,
            LevelType::High => out.write_all(b"High")?,
            LevelType::Urgent => out.write_all(b"Urgent")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Leveltype, Pg> for LevelType {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Low" => Ok(LevelType::Low),
            b"Medium" => Ok(LevelType::Medium),
            b"High" => Ok(LevelType::High),
            b"Urgent" => Ok(LevelType::Urgent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

