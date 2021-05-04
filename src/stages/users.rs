use crate::{requests::api_manager::ApiManager, RobberError};
use rusqlite::params;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::serde_as;

macro_rules! try_save {
    ($obj:expr, $name:ident, $conn:expr, $table_name:expr, $id:expr) => {
        if let Some(obj) = $obj {
            if let Err(e) = obj.store($conn, $table_name, $id) {
                panic!("Failed saving {}: {}", stringify!($name), e);
            }
        }
    };
}

macro_rules! store_many {
    ($values:expr, $conn:expr, $table_name:expr, $user_id:expr) => {{
        let mut total_length: usize = 0;
        for value in $values {
            let operation_result = value.store($conn, $table_name, $user_id);
            if let Ok(length) = operation_result {
                total_length += length;
            } else if let Err(e) = operation_result {
                return Err(e);
            } else {
                unreachable!()
            }
        }
        Ok(total_length)
    }};
}

pub trait StoreExt {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error>;
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct CareerInfo {
    group_id: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    company: Option<String>,
    country_id: Option<i64>,
    city_id: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    city_name: Option<String>,
    from: Option<i64>,
    until: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    position: Option<String>,
}

impl StoreExt for CareerInfo {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!("INSERT OR REPLACE INTO {} (user_id, group_id, company, country_id, city_id, city_name, \"from\", \"until\", position) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", table_name);
        connection.execute(
            &query,
            params![
                user_id,
                self.group_id,
                self.company,
                self.country_id,
                self.city_id,
                self.city_name,
                self.from,
                self.until,
                self.position
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct City {
    id: i64,
    title: String,
}

impl StoreExt for City {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!(
            "INSERT OR REPLACE INTO {} (user_id, id, title) VALUES (?, ?, ?)",
            table_name
        );
        connection.execute(&query, params![user_id, self.id, self.title])
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Counters {
    albums: i64,
    videos: i64,
    audios: i64,
    photos: i64,
    notes: i64,
    friends: i64,
    groups: i64,
    user_videos: i64,
    followers: i64,
    pages: i64,
}

impl StoreExt for Counters {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!("INSERT OR REPLACE INTO {} (user_id, albums, videos, audios, photos, notes, friends, groups, user_videos, followers, pages) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", table_name);
        connection.execute(
            &query,
            params![
                user_id,
                self.albums,
                self.videos,
                self.audios,
                self.photos,
                self.notes,
                self.friends,
                self.groups,
                self.user_videos,
                self.followers,
                self.pages
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    id: i64,
    title: String,
}

impl StoreExt for Country {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!(
            "INSERT OR REPLACE INTO {} (user_id, id, title) VALUES (?, ?, ?)",
            table_name
        );
        connection.execute(&query, params![user_id, self.id, self.title])
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct EducationInfo {
    university: i64,
    university_name: String,
    faculty: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    faculty_name: Option<String>,
    graduation: Option<i64>,
}

impl StoreExt for EducationInfo {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!("INSERT OR REPLACE INTO {} (user_id, university, university_name, faculty, faculty_name, graduation) VALUES (?, ?, ?, ?, ?, ?)", table_name);
        connection.execute(
            &query,
            params![
                user_id,
                self.university,
                self.university_name,
                self.faculty,
                self.faculty_name,
                self.graduation
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastSeen {
    time: i64,
    platform: i64,
}

impl StoreExt for LastSeen {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!(
            "INSERT OR REPLACE INTO {} (user_id, time, platform) VALUES (?, ?, ?)",
            table_name
        );
        connection.execute(&query, params![user_id, self.time, self.platform])
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MilitaryInfo {
    unit: String,
    unit_id: i64,
    country_id: i64,
    from: Option<i64>,
    until: Option<i64>,
}

impl StoreExt for MilitaryInfo {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!("INSERT OR REPLACE INTO {} (user_id, unit, unit_id, country_id, \"from\", \"until\") VALUES (?, ?, ?, ?, ?, ?)", table_name);
        connection.execute(
            &query,
            params![
                user_id,
                self.unit,
                self.unit_id,
                self.country_id,
                self.from,
                self.until
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Occupation {
    #[serde(rename = "type")]
    r#type: String,
    id: i64,
    name: String,
}

impl StoreExt for Occupation {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!(
            "INSERT OR REPLACE INTO {} (user_id, type, id, name) VALUES (?, ?, ?, ?)",
            table_name
        );
        connection.execute(&query, params![user_id, self.r#type, self.id, self.name])
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct PersonalInfo {
    political: Option<i64>,
    langs: Option<Vec<String>>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    religion: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    inspired_by: Option<String>,
    people_main: Option<i64>,
    life_main: Option<i64>,
    smoking: Option<i64>,
    alcohol: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Personal {
    Value(PersonalInfo),
    None(Vec<()>),
}

impl StoreExt for Personal {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        match self {
            Personal::Value(value) => {
                let query = format!("INSERT OR REPLACE INTO {} (user_id, political, langs, religion, inspired_by, people_main, life_main, smoking, alcohol) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", table_name);
                let langs = value.langs.map(|e| e.join(", "));

                connection.execute(
                    &query,
                    params![
                        user_id,
                        value.political,
                        langs,
                        value.religion,
                        value.inspired_by,
                        value.people_main,
                        value.life_main,
                        value.smoking,
                        value.alcohol
                    ],
                )
            }
            Personal::None(_) => Ok(0),
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Relative {
    id: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    name: Option<String>,
    #[serde(rename = "type")]
    r#type: String,
}

impl StoreExt for Relative {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!(
            "INSERT OR REPLACE INTO {} (user_id, id, name, type) VALUES (?, ?, ?, ?)",
            table_name
        );

        connection.execute(&query, params![user_id, self.id, self.name, self.r#type])
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Relatives {
    Value(Vec<Relative>),
}

impl StoreExt for Relatives {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        match self {
            Relatives::Value(e) => {
                let mut total_length: usize = 0;
                for career in e {
                    let operation_result = career.store(connection, table_name, user_id);
                    if let Ok(length) = operation_result {
                        total_length += length;
                    } else if let Err(e) = operation_result {
                        return Err(e);
                    } else {
                        unreachable!()
                    }
                }
                Ok(total_length)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelationPartner {
    id: i64,
    first_name: String,
    last_name: String,
}

impl StoreExt for RelationPartner {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!(
            "INSERT OR REPLACE INTO {} (user_id, id, first_name, last_name) VALUES (?, ?, ?, ?)",
            table_name
        );

        connection.execute(
            &query,
            params![user_id, self.id, self.first_name, self.last_name],
        )
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct School {
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    id: Option<String>,
    country: i64,
    city: i64,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    name: Option<String>,
    year_from: Option<i64>,
    year_to: Option<i64>,
    year_graduated: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    class: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    speciality: Option<String>,
    #[serde(rename = "type")]
    r#type: Option<i64>,
}

impl StoreExt for School {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!("INSERT OR REPLACE INTO {} (user_id, id, country, city, name, \"year_from\", year_to, year_graduated, class, speciality, type) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", table_name);

        connection.execute(
            &query,
            params![
                user_id,
                self.id,
                self.country,
                self.city,
                self.name,
                self.year_from,
                self.year_to,
                self.year_graduated,
                self.class,
                self.speciality,
                self.r#type
            ],
        )
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Contacts {
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    mobile_phone: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    home_phone: Option<String>,
}

impl StoreExt for Contacts {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        if self.mobile_phone.is_some() || self.home_phone.is_some() {
            let query = format!(
                "INSERT OR REPLACE INTO {} (user_id, mobile_phone, home_phone) VALUES (?, ?, ?)",
                table_name
            );

            connection.execute(&query, params![user_id, self.mobile_phone, self.home_phone])
        } else {
            Ok(0)
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct University {
    id: i64,
    country: i64,
    city: i64,
    name: String,
    faculty: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    faculty_name: Option<String>,
    chair: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    chair_name: Option<String>,
    graduation: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    education_form: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    education_status: Option<String>,
}

impl StoreExt for University {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        let query = format!("INSERT OR REPLACE INTO {} (user_id, id, country, city, name, faculty, faculty_name, chair, chair_name, graduation, education_form, education_status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", table_name);

        connection.execute(
            &query,
            params![
                user_id,
                self.id,
                self.country,
                self.city,
                self.name,
                self.faculty,
                self.faculty_name,
                self.chair,
                self.chair_name,
                self.graduation,
                self.education_form,
                self.education_status
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Career {
    One(CareerInfo),
    Many(Vec<CareerInfo>),
}

impl StoreExt for Career {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        match self {
            Career::One(e) => e.store(connection, table_name, user_id),
            Career::Many(e) => store_many!(e, connection, table_name, user_id),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Universities {
    Value(Vec<University>),
}

impl StoreExt for Universities {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        match self {
            Universities::Value(e) => store_many!(e, connection, table_name, user_id),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Schools {
    Value(Vec<School>),
}

impl StoreExt for Schools {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        match self {
            Schools::Value(e) => store_many!(e, connection, table_name, user_id),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Military {
    One(MilitaryInfo),
    Many(Vec<MilitaryInfo>),
}

impl StoreExt for Military {
    fn store(
        self,
        connection: &rusqlite::Connection,
        table_name: &str,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        match self {
            Military::One(e) => e.store(connection, table_name, user_id),
            Military::Many(e) => store_many!(e, connection, table_name, user_id),
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: i64,

    first_name: String,
    last_name: String,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    deactivated: Option<String>,
    is_closed: Option<bool>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    about: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    activities: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    bdate: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    books: Option<String>,
    career: Option<Career>,
    city: Option<City>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    skype: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    facebook: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    twitter: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    livejournal: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    instagram: Option<String>,

    #[serde(flatten)]
    contacts: Option<Contacts>,

    counters: Option<Counters>,
    country: Option<Country>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    domain: Option<String>,
    education: Option<EducationInfo>,
    followers_count: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    games: Option<String>,

    has_mobile: Option<i64>,
    has_photo: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    home_town: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    interests: Option<String>,
    last_seen: Option<LastSeen>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    maiden_name: Option<String>,
    military: Option<Military>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    movies: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    music: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    nickname: Option<String>,
    occupation: Option<Occupation>,
    personal: Option<Personal>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    photo_max_orig: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    quotes: Option<String>,
    relatives: Option<Relatives>,

    relation: Option<i64>,
    relation_partner: Option<RelationPartner>,
    schools: Option<Schools>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    screen_name: Option<String>,
    sex: Option<i64>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    site: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    status: Option<String>,

    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    #[serde(default)]
    tv: Option<String>,
    universities: Option<Universities>,
    verified: Option<i64>,
}

impl User {
    pub fn store(self, connection: &rusqlite::Connection, table_name: &str) {
        let query = format!("INSERT OR REPLACE INTO {} (id, first_name, last_name, deactivated, is_closed, about, activities, bdate, books, domain, followers_count, games, has_mobile, has_photo, home_town, interests, maiden_name, movies, music, nickname, photo_max_orig, quotes, screen_name, sex, site, status, tv, verified, skype, facebook, twitter, livejournal, instagram) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", table_name);

        if let Err(e) = connection.execute(
            &query,
            params![
                self.id,
                self.first_name,
                self.last_name,
                self.deactivated,
                self.is_closed,
                self.about,
                self.activities,
                self.bdate,
                self.books,
                self.domain,
                self.followers_count,
                self.games,
                self.has_mobile,
                self.has_photo,
                self.home_town,
                self.interests,
                self.maiden_name,
                self.movies,
                self.music,
                self.nickname,
                self.photo_max_orig,
                self.quotes,
                self.screen_name,
                self.sex,
                self.site,
                self.status,
                self.tv,
                self.verified,
                self.skype,
                self.facebook,
                self.twitter,
                self.livejournal,
                self.instagram
            ],
        ) {
            panic!("Failed saving object: {}", e);
        }

        try_save!(self.career, career, connection, "career", self.id);
        try_save!(self.city, city, connection, "city", self.id);

        try_save!(self.counters, counters, connection, "counters", self.id);
        try_save!(self.country, country, connection, "country", self.id);

        try_save!(self.education, education, &connection, "education", self.id);

        try_save!(self.last_seen, last_seen, &connection, "last_seen", self.id);
        try_save!(self.personal, personal, &connection, "personal", self.id);
        try_save!(self.contacts, contacts, &connection, "contacts", self.id);

        try_save!(self.military, military, connection, "military", self.id);
        try_save!(
            self.occupation,
            occupation,
            connection,
            "occupation",
            self.id
        );
        try_save!(self.relatives, relatives, connection, "relatives", self.id);

        try_save!(
            self.relation_partner,
            relation_partner,
            connection,
            "relation_partner",
            self.id
        );
        try_save!(self.schools, schools, connection, "schools", self.id);
        try_save!(
            self.universities,
            universities,
            connection,
            "universities",
            self.id
        );
    }
}

#[derive(Deserialize)]
pub struct UserGet {
    response: Vec<User>,
}

impl User {
    pub async fn from_page(
        api: &ApiManager,
        user_id: &str,
        fields: &str,
    ) -> Result<User, RobberError> {
        let result = User::from_pages(api, user_id, fields).await;
        if let Ok(mut e) = result {
            Ok(e.pop().unwrap())
        } else if let Err(e) = result {
            Err(e)
        } else {
            unreachable!()
        }
    }
    pub async fn from_pages(
        api: &ApiManager,
        user_id: &str,
        fields: &str,
    ) -> Result<Vec<User>, RobberError> {
        let resp = api
            .request_json::<_, UserGet>("users.get", &[("user_ids", user_id), ("fields", fields)])
            .await
            .unwrap();
        Ok(resp.response)
    }
}

impl std::str::FromStr for User {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
