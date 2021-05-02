use std::fmt::Debug;

use serde::Deserialize;

use crate::{requests::api_manager::ApiManager, RobberError};

#[derive(Debug, Deserialize)]
pub struct CareerInfo {
    group_id: Option<i32>,
    company: Option<String>,
    country_id: i32,
    city_id: Option<i32>,
    city_name: Option<String>,
    from: i32,
    until: Option<i32>,
    position: String,
}

#[derive(Debug, Deserialize)]
pub struct City {
    id: i32,
    title: String,
}

#[derive(Debug, Deserialize)]
pub struct Counters {
    albums: i32,
    videos: i32,
    audios: i32,
    photos: i32,
    notes: i32,
    friends: i32,
    groups: i32,
    online_friends: i32,
    user_videos: i32,
    followers: i32,
    pages: i32,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    id: i32,
    title: String,
}

#[derive(Debug, Deserialize)]
pub struct EducationInfo {
    university: i32,
    university_name: String,
    faculty: Option<i32>,
    faculty_name: Option<String>,
    graduation: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct LastSeen {
    time: i64,
    platform: i32,
}

#[derive(Debug, Deserialize)]
pub struct MilitaryInfo {
    unit: String,
    unit_id: i32,
    country_id: i32,
    from: Option<i32>,
    option: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Occupation {
    r#type: String,
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Personal {
    political: Option<i32>,
    langs: Option<Vec<String>>,
    religion: Option<String>,
    inspired_by: Option<String>,
    people_main: Option<i32>,
    life_main: Option<i32>,
    smoking: Option<i32>,
    alcohol: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Relative {
    id: Option<i32>,
    name: String,
    r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct RelationPartner {
    first_name: String,
    id: i32,
    last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct School {
    id: i32,
    country: i32,
    city: i32,
    name: String,
    year_from: Option<i32>,
    year_to: Option<i32>,
    year_graduated: Option<i32>,
    class: Option<String>,
    speciality: Option<String>,
    r#type: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct University {
    id: i32,
    country: i32,
    city: i32,
    name: String,
    faculty: Option<i32>,
    faculty_name: Option<String>,
    chair: Option<i32>,
    chair_name: Option<String>,
    graduation: Option<i32>,
    education_from: Option<String>,
    education_status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Contacts {
    mobile_phone: Option<String>,
    home_phone: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Career {
    One(CareerInfo),
    Many(Vec<CareerInfo>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Education {
    One(EducationInfo),
    Many(Vec<EducationInfo>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Military {
    One(MilitaryInfo),
    Many(Vec<MilitaryInfo>),
}

#[derive(Debug, Deserialize)]
pub struct User {
    id: i32,

    first_name: String,
    last_name: String,
    deactivated: Option<String>,
    is_closed: bool,

    about: Option<String>,
    activities: Option<String>,
    bdate: Option<String>,
    books: Option<String>,
    career: Option<Career>,
    city: Option<City>,

    skype: Option<String>,
    facebook: Option<String>,
    twitter: Option<String>,
    livejournal: Option<String>,
    instagram: Option<String>,

    contacts: Option<Contacts>,
    counters: Option<Counters>,
    country: Option<Country>,

    domain: Option<String>,
    education: Option<Education>,
    followers_count: Option<i32>,

    games: Option<String>,

    has_mobile: Option<i32>,
    has_photo: Option<i32>,

    home_town: Option<String>,
    interests: Option<String>,
    last_seen: Option<LastSeen>,
    maiden_name: Option<String>,
    military: Option<Military>,
    movies: Option<String>,
    music: Option<String>,
    nickname: Option<String>,
    occupation: Option<Occupation>,
    personal: Option<Personal>,
    photo_max_orig: Option<String>,
    quotes: Option<String>,
    relatives: Option<Vec<Relative>>,

    relation: Option<i32>,
    relation_partner: Option<RelationPartner>,
    school: Option<Vec<School>>,
    screen_name: Option<String>,
    sex: Option<i32>,
    site: Option<String>,
    status: Option<String>,
    tv: Option<String>,
    universities: Option<Vec<University>>,
    verified: Option<i32>,
}

#[derive(Deserialize)]
pub struct UserGet {
    response: Vec<User>,
}

impl User {
    pub fn from_str(data: &str) -> serde_json::Result<User> {
        serde_json::from_str(data)
    }
    pub async fn from_page(
        api: &ApiManager,
        user_id: &str,
        fields: &str,
    ) -> Result<User, RobberError> {
        let mut resp = api
            .request_json::<_, UserGet>("users.get", &[("user_ids", user_id), ("fields", fields)])
            .await
            .unwrap();
        Ok(resp.response.pop().unwrap())
    }
}
