use cute_fox::{
    requests::api_manager::{ApiManager, API_VERSION},
    stages::users::User,
};

#[tokio::main]
async fn main() {
    let mut args = std::env::args();

    let _ = args.next().unwrap();
    let access_token = args.next().expect("Please, specify argument: ACCESS_TOKEN");
    let user_id = args.next().expect("Please, specify argument: USER_ID");

    let api_manager = ApiManager::new(access_token, API_VERSION);
    let user = User::from_page(&api_manager, &user_id, "relatives").await;

    println!("{:#?}", user.unwrap());
}
