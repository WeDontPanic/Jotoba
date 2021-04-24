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
pub enum Field {
    #[strum(serialize = "agric")]
    Agriculture,
    #[strum(serialize = "anat")]
    Anatomy,
    #[strum(serialize = "archeol")]
    Archeology,
    #[strum(serialize = "archit")]
    Architecture,
    #[strum(serialize = "art")]
    ArtAesthetics,
    #[strum(serialize = "astron")]
    Astronomy,
    #[strum(serialize = "audvid")]
    AudioVisual,
    #[strum(serialize = "aviat")]
    Aviation,
    #[strum(serialize = "baseb")]
    Baseball,
    #[strum(serialize = "biochem")]
    Biochemistry,
    #[strum(serialize = "biol")]
    Biology,
    #[strum(serialize = "bot")]
    Botany,
    #[strum(serialize = "Buddh")]
    Buddhism,
    #[strum(serialize = "bus")]
    Business,
    #[strum(serialize = "chem")]
    Chemistry,
    #[strum(serialize = "Christn")]
    Christianity,
    #[strum(serialize = "comp")]
    Computing,
    #[strum(serialize = "cryst")]
    Crystallography,
    #[strum(serialize = "ecol")]
    Ecology,
    #[strum(serialize = "econ")]
    Economics,
    #[strum(serialize = "elec")]
    Electricity,
    #[strum(serialize = "electr")]
    Electronics,
    #[strum(serialize = "embryo")]
    Embryology,
    #[strum(serialize = "engr")]
    Engineering,
    #[strum(serialize = "ent")]
    Entomology,
    #[strum(serialize = "finc")]
    Finance,
    #[strum(serialize = "fish")]
    Fishing,
    #[strum(serialize = "food")]
    FoodCooking,
    #[strum(serialize = "gardn")]
    Gardening,
    #[strum(serialize = "genet")]
    Genetics,
    #[strum(serialize = "geogr")]
    Geography,
    #[strum(serialize = "geol")]
    Geology,
    #[strum(serialize = "geom")]
    Geometry,
    #[strum(serialize = "go")]
    GoGame,
    #[strum(serialize = "golf")]
    Golf,
    #[strum(serialize = "gramm")]
    Grammar,
    #[strum(serialize = "grmyth")]
    GreekMythology,
    #[strum(serialize = "hanaf")]
    Hanafuda,
    #[strum(serialize = "horse")]
    Horseracing,
    #[strum(serialize = "law")]
    Law,
    #[strum(serialize = "ling")]
    Linguistics,
    #[strum(serialize = "logic")]
    Logic,
    #[strum(serialize = "MA")]
    MartialArts,
    #[strum(serialize = "mahj")]
    Mahjong,
    #[strum(serialize = "math")]
    Mathematics,
    #[strum(serialize = "mech")]
    MechanicalEngineering,
    #[strum(serialize = "med")]
    Medicine,
    #[strum(serialize = "met")]
    ClimateWeather,
    #[strum(serialize = "mil")]
    Military,
    #[strum(serialize = "music")]
    Music,
    #[strum(serialize = "ornith")]
    Ornithology,
    #[strum(serialize = "paleo")]
    Paleontology,
    #[strum(serialize = "pathol")]
    Pathology,
    #[strum(serialize = "pharm")]
    Pharmacy,
    #[strum(serialize = "phil")]
    Philosophy,
    #[strum(serialize = "photo")]
    Photography,
    #[strum(serialize = "physics")]
    Physics,
    #[strum(serialize = "physiol")]
    Physiology,
    #[strum(serialize = "print")]
    Printing,
    #[strum(serialize = "psych")]
    PsychologyPsychiatry,
    #[strum(serialize = "Shinto")]
    Shinto,
    #[strum(serialize = "shogi")]
    Shogi,
    #[strum(serialize = "sports")]
    Sports,
    #[strum(serialize = "stat")]
    Statistics,
    #[strum(serialize = "sumo")]
    Sumo,
    #[strum(serialize = "telec")]
    Telecommunications,
    #[strum(serialize = "tradem")]
    Trademark,
    #[strum(serialize = "vidg")]
    Videogame,
    #[strum(serialize = "zool")]
    Zoology,
}

impl Field {
    pub fn humanize(&self) -> String {
        // TODO generate proper string
        format!("{:?}", self)
    }
}

impl ToSql<Text, Pg> for Field {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <&str as ToSql<Text, Pg>>::to_sql(&self.as_ref(), out)
    }
}

impl FromSql<Text, Pg> for Field {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::from_str(
            (<String as FromSql<Text, Pg>>::from_sql(bytes)?).as_str(),
        )?)
    }
}
