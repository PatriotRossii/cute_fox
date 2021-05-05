use cute_fox::{
    requests::api_manager::{ApiManager, API_VERSION},
    stages::groups::GroupInteraction,
};

#[tokio::main]
async fn main() {
    let mut args = std::env::args();

    let _ = args.next().unwrap();
    let access_token = args.next().expect("Please, specify argument: ACCESS_TOKEN");
    let group_id = args.next().expect("Please, specify argument: GROUP_ID");

    let group_id = group_id.parse().expect("Please, specify correct group id");

    let api = ApiManager::new(access_token, API_VERSION);
    let members = api.get_members(group_id, "").await;

    println!("{:#?}", members.unwrap());
}
