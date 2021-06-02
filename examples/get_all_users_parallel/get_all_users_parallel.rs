use clap::{App, Arg};
use cute_fox::{requests::api_manager::API_VERSION, CuteExecutor, CuteFox, CuteTask};

pub fn is_integer(x: String) -> Result<(), String> {
    match x.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Expected integer, found shit"))
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("get all users parallel")
        .author("PatriotRossii <patriotrossii2019@mail.ru")
        .arg(
            Arg::with_name("lower_bound")
                .long("lower_bound")
                .value_name("LOWER_BOUND")
                .takes_value(true)
                .required(true)
                .help("Upper bound of ids to collect")
                .validator(is_integer)
        )
        .arg(
            Arg::with_name("upper_bound")
                .long("upper_bound")
                .value_name("UPPER_BOUND")
                .takes_value(true)
                .required(true)
                .help("Upper bound of ids to collect")
                .validator(is_integer)
        )
        .arg(
            Arg::with_name("field")
                .long("field")
                .value_name("FIELD")
                .takes_value(true)
                .help("Field to collect")
                .multiple(true)
        )
        .arg(
            Arg::with_name("access_token")
                .long("access_token")
                .value_name("ACCESS_TOKEN")
                .takes_value(true)
                .help("Access token to use")
                .multiple(true)
                .required(true)
        )
        .get_matches();

    let from: i32 = matches.value_of("lower_bound").unwrap().parse().unwrap();
    let to: i32 = matches.value_of("upper_bound").unwrap().parse().unwrap();
    
    let fields: String = match matches.values_of("field") {
        Some(e) => e.collect::<Vec<&str>>().join(","),
        None => String::from("")
    };
    let tokens: Vec<String> = matches.values_of("access_token").unwrap().map(|x| x.to_string()).collect::<Vec<String>>();

    let fox = CuteFox::new(&tokens, API_VERSION);
    let task = CuteTask::GetUsers {
        user_ids: (from..to).collect::<Vec<i32>>(),
        fields,
    };
    fox.execute(task).await.unwrap();
}
