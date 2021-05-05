use cute_fox::{requests::api_manager::{ApiManager, API_VERSION}, stages::groups::{GroupInteraction}};
use rusqlite::Connection;

const FIELDS: &str = "verified, sex, bdate, city, country, home_town, has_photo, photo_max_orig, domain, has_mobile, contacts, site, education, universities, schools, status, last_seen, followers_count, occupation, nickname, relatives, relation, personal, connections, activities, interests, music, movies, tv, books, games, about, quotes, timezone, screen_name, maiden_name, career, military";

#[tokio::main]
async fn main() {
    let mut args = std::env::args();

    let _ = args.next().unwrap();
    let db_path = args.next().expect("Please, specify argument: DB_PATH");
    let access_token = args.next().expect("Please, specify argument: ACCESS_TOKEN");
    let group_id = args.next().expect("Please, specify argument: GROUP_ID");

    let group_id = group_id.parse().expect("Please, specify correct group id");

    let api = ApiManager::new(access_token, API_VERSION);
    let members = api.get_members(group_id, FIELDS).await;

    let mut connection = Connection::open(&db_path).expect("Failed to open database");

    let tx = connection.transaction().unwrap();
    for member in members.unwrap() {
        member.store(&tx, "objects");
    }
    tx.commit().unwrap();
}
