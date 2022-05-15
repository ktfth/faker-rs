#[derive(Clone)]
pub struct Locale {
    pub lorem: Vec<&'static str>,

    pub image_categories: Vec<&'static str>,

    pub name_first: Vec<&'static str>,
    pub name_last: Vec<&'static str>,
    pub name_prefix: Vec<&'static str>,
    pub name_suffix: Vec<&'static str>,
    pub name_title: Vec<&'static str>,

    pub phone_formats: Vec<&'static str>,

    pub building_number: Vec<&'static str>,
    pub city_prefix: Vec<&'static str>,
    pub city_suffix: Vec<&'static str>,
    pub street_suffix: Vec<&'static str>,
    pub secondary_address: Vec<&'static str>,
    pub state: Vec<&'static str>,
    pub zip: Vec<&'static str>,
    pub time_zone: Vec<&'static str>,
    pub state_abbr: Vec<&'static str>,
    pub country: Vec<&'static str>,
}

impl Locale {
    pub fn load(locale: &str) -> Self {
        match locale {
            "en" => en::load(),
            "pt_BR" => pt_br::load(),
            _ => unimplemented!(),
        }
    }
}

pub mod en;
pub mod pt_br;
mod es;
mod fr;
