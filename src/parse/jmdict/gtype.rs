use std::{io::Write, str::FromStr};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Text,
    types::{FromSql, ToSql},
};

use strum_macros::{AsRefStr, EnumString};

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
#[sql_type = "Text"]
pub enum GType {
    #[strum(serialize = "lit")]
    Literal,
    #[strum(serialize = "fig")]
    Figurative,
    #[strum(serialize = "expl")]
    Explanation,
}

impl ToSql<Text, Pg> for GType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <&str as ToSql<Text, Pg>>::to_sql(&self.as_ref(), out)
    }
}

impl FromSql<Text, Pg> for GType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::from_str(
            (<String as FromSql<Text, Pg>>::from_sql(bytes)?).as_str(),
        )?)
    }
}
