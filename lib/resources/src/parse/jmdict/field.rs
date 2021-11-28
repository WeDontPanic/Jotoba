use localization::{language::Language, traits::Translatable, TranslationDict};
use strum_macros::{AsRefStr, EnumString};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Serialize, Deserialize, Hash)]
#[repr(u8)]
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
    #[strum(serialize = "cloth")]
    Clothing,
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
    Psychology,
    #[strum(serialize = "psy")]
    Psychitatry,
    #[strum(serialize = "Shinto")]
    Shinto,
    #[strum(serialize = "rail")]
    Railway,
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

impl Translatable for Field {
    fn get_id(&self) -> &'static str {
        match self {
            Field::Agriculture => "Agriculture",
            Field::Anatomy => "Anatomy",
            Field::Archeology => "Archeology",
            Field::Architecture => "Architecture",
            Field::ArtAesthetics => "Art aesthetics",
            Field::Astronomy => "Astronomy",
            Field::AudioVisual => "Audio/visual",
            Field::Aviation => "Aviation",
            Field::Baseball => "Baseball",
            Field::Biochemistry => "Biochemistry",
            Field::Biology => "Biology",
            Field::Botany => "Botany",
            Field::Buddhism => "Buddhism",
            Field::Business => "Business",
            Field::Chemistry => "Chemistry",
            Field::Christianity => "Christianity",
            Field::Computing => "Computing",
            Field::Crystallography => "Crystallography",
            Field::Ecology => "Ecology",
            Field::Economics => "Economics",
            Field::Electricity => "Electricity",
            Field::Electronics => "Electronics",
            Field::Embryology => "Embryology",
            Field::Engineering => "Engineering",
            Field::Entomology => "Entomology",
            Field::Finance => "Finance",
            Field::Fishing => "Fishing",
            Field::FoodCooking => "FoodCooking",
            Field::Gardening => "Gardening",
            Field::Genetics => "Genetics",
            Field::Geography => "Geography",
            Field::Geology => "Geology",
            Field::Geometry => "Geometry",
            Field::GoGame => "Go (game)",
            Field::Golf => "Golf",
            Field::Grammar => "Grammar",
            Field::GreekMythology => "Greek mythology",
            Field::Hanafuda => "Hanafuda",
            Field::Horseracing => "Horseracing",
            Field::Law => "Law",
            Field::Linguistics => "Linguistics",
            Field::Logic => "Logic",
            Field::MartialArts => "Martial arts",
            Field::Mahjong => "Mahjong",
            Field::Mathematics => "Mathematics",
            Field::MechanicalEngineering => "MechanicalEngineering",
            Field::Medicine => "Medicine",
            Field::ClimateWeather => "Climate/weather",
            Field::Military => "Military",
            Field::Music => "Music",
            Field::Ornithology => "Ornithology",
            Field::Paleontology => "Paleontology",
            Field::Pathology => "Pathology",
            Field::Pharmacy => "Pharmacy",
            Field::Philosophy => "Philosophy",
            Field::Photography => "Photography",
            Field::Physics => "Physics",
            Field::Physiology => "Physiology",
            Field::Printing => "Printing",
            Field::Psychology => "Psychology",
            Field::Psychitatry => "Psychiatry",
            Field::Railway => "Railway",
            Field::Shinto => "Shinto",
            Field::Shogi => "Shogi",
            Field::Sports => "Sports",
            Field::Statistics => "Statistics",
            Field::Sumo => "Sumo",
            Field::Telecommunications => "Telecommunications",
            Field::Trademark => "Trademark",
            Field::Videogame => "Videogame",
            Field::Zoology => "Zoology",
            Field::Clothing => "Clothing",
        }
    }

    // Translate to eg "Zoology term"
    fn gettext_custom(&self, dict: &TranslationDict, language: Option<Language>) -> String {
        dict.gettext_fmt("{} term", &[self.gettext(dict, language)], language)
    }
}
