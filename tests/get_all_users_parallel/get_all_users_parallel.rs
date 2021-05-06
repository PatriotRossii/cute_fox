use cute_fox::{requests::api_manager::API_VERSION, CuteExecutor, CuteFox, CuteTask};

#[tokio::main]
async fn main() {
    let mut args = std::env::args();

    let _ = args.next().unwrap();

    let from = args
        .next()
        .expect("Please, specify start user_id")
        .parse::<i32>()
        .expect("Please, specify correct user_id");
    let to = args
        .next()
        .expect("Please, specify end user_id")
        .parse::<i32>()
        .expect("Please, specify correct user_id");

    let fields = args.next().expect("Please, specify fields need to collect");
    let tokens: Vec<String> = args.collect();

    if tokens.is_empty() {
        panic!("Please, specify at least one token")
    }

    let fox = CuteFox::new(&tokens, API_VERSION);
    let task = CuteTask::GetUsers {
        user_ids: (from..=to).collect::<Vec<i32>>(),
        fields,
    };
    fox.execute(task).await.unwrap();
}
