use cute_fox::{
    requests::api_manager::{ApiManager, API_VERSION},
    stages::groups::Group,
};

#[tokio::main]
async fn main() {
    let mut args = std::env::args();

    let _ = args.next().unwrap();
    let access_token = args.next().expect("Please, specify argument: ACCESS_TOKEN");
    let group_id = args.next().expect("Please, specify argument: GROUP_ID");

    let group_id = group_id.parse().expect("Please, specify correct group id");

    let api_manager = ApiManager::new(access_token, API_VERSION);
    let members = Group::get_members(&api_manager, group_id, "").await;

    println!("{:#?}", members.unwrap());
}
