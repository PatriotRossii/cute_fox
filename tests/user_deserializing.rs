use cute_fox::stages::users::User;

#[test]
fn test_deserialize() {
    let data = r#"
    {
        "first_name": "Лисид",
        "id": 544923642,
        "last_name": "Лаконский",
        "can_access_closed": true,
        "is_closed": false,
        "sex": 2,
        "screen_name": "scoped_lock",
        "verified": 0,
        "nickname": "",
        "domain": "scoped_lock",
        "bdate": "26.7.2004",
        "country": {
        "id": 1,
        "title": "Россия"
        },
        "timezone": 5,
        "photo_max_orig": "https://sun3-12.u...2,290,290&ava=1",
        "photo_id": "544923642_457245176",
        "has_photo": 1,
        "has_mobile": 1,
        "skype": "d",
        "interests": "",
        "books": "",
        "tv": "",
        "quotes": "",
        "about": "",
        "games": "",
        "movies": "",
        "activities": "Подготовка к ЕГЭ по физике, профильной математике, информатике и русскому языку.",
        "music": "",
        "mobile_phone": "79825469768",
        "home_phone": "",
        "site": "",
        "status_audio": {
        "artist": "Lord Of The Lost",
        "id": 456239312,
        "owner_id": 544923642,
        "title": "Drag Me to Hell",
        "duration": 25,
        "is_explicit": false,
        "is_focus_track": false,
        "track_code": "f74338a50BBDy0fMlVD2yjaFJtPlYFC0lzPvQIOY_5EWVWx6",
        "url": "https://vk.com/mp...api_unavailable.mp3",
        "date": 1613646074,
        "main_artists": [{
        "name": "Lord Of The Lost",
        "domain": "4996489848628410909",
        "id": "4996489848628410909"
        }],
        "short_videos_allowed": false,
        "stories_allowed": false,
        "stories_cover_allowed": false
        },
        "status": "Умный пуффыстик :3",
        "last_seen": {
        "platform": 7,
        "time": 1619955329
        },
        "followers_count": 10,
        "occupation": {
        "id": 2,
        "name": "МГУ",
        "type": "university"
        },
        "career": [],
        "military": [],
        "university": 2,
        "university_name": "МГУ",
        "faculty": 0,
        "faculty_name": "",
        "graduation": 0,
        "home_town": "",
        "relation": 7,
        "personal": {
        "alcohol": 0,
        "inspired_by": "Честно говоря, не знаю",
        "langs": ["Русский"],
        "life_main": 6,
        "people_main": 2,
        "smoking": 2
        },
        "universities": [{
        "city": 0,
        "country": 1,
        "id": 2,
        "name": "МГУ"
        }],
        "schools": [],
        "relatives": []
        }"#;

    let user: User = serde_json::from_str(data).unwrap();
    println!("{:#?}", user);
}
