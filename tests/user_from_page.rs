use cute_fox::{
    requests::api_manager::{ApiManager, API_VERSION},
    stages::users::UserInteraction,
};

#[tokio::main]
async fn main() {
    let mut args = std::env::args();

    let _ = args.next().unwrap();
    let access_token = args.next().expect("Please, specify argument: ACCESS_TOKEN");
    let user_id = args.next().expect("Please, specify argument: USER_ID");

    let user_id = user_id.parse().expect("Please, specify correct user id");

    let api = ApiManager::new(access_token, API_VERSION);
    let user = api.get_user(user_id, "relatives").await;

    println!("{:#?}", user.unwrap());
}
