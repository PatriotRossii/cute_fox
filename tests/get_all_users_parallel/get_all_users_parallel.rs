use cute_fox::{requests::api_manager::API_VERSION, CuteExecutor, CuteFox, CuteTask};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let tokens = vec![
        String::from(
            "1a75deefcde10db3cbe49f350d505359939488642b3c4254af12382bcfabbdaee539d72d16e3090d450e9",
        ),
        String::from(
            "004d3a7c93b131021e13925aca95ceb6db27c772ec6cbf55978356e681d5a298bd1f8eb747b4613a255ae",
        ),
    ];
    let fox = CuteFox::new(&tokens, API_VERSION);
    let task = CuteTask::GetUsers {
        user_ids: (0..10000).collect::<Vec<i32>>(),
        fields: String::from(""),
    };

    let start = Instant::now();
    fox.execute(task).await.unwrap();
    println!(
        "Time elapsed in expensive_function() is: {:?}",
        start.elapsed()
    );
}
