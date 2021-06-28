use announcer::db;

#[derive(PartialEq, sqlx::FromRow, Debug)]
struct Sound {
    id: i64,
    file_name: String,
    display_name: String,
}

#[async_std::test]
async fn connect_to_db() {
    let pool = db::connect(None).await.unwrap();

    let row = sqlx::query_as::<_, Sound>("select * from sound")
        .fetch_one(&pool)
        .await
        .unwrap();

    let expected_sound = Sound {
        id: 0,
        file_name: String::from("soft_bells.mp3"),
        display_name: String::from("soft bells"),
    };

    k9::assert_equal!(row, expected_sound);
}
