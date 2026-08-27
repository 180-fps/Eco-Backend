use std::env;

pub struct Config {
    pub port: u16,
    pub season_number: f64,
    #[allow(dead_code)]
    pub season_template_id: String,
    pub season_begin: String,
    pub season_end: String,
    pub season_displayed_end: String,
    pub matchmaker_ip: String,
    pub game_servers: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let season_number: f64 = env::var("SEASON_NUMBER")
            .unwrap_or_else(|_| "12.41".into())
            .parse()
            .expect("SEASON_NUMBER must be a valid number");

        let season_int = season_number.floor() as u32;
        let season_template_id = format!("AthenaSeason:athenaseason{}", season_int);

        let season_begin = match season_int {
            1  => "2017-10-26T00:00:00Z",
            2  => "2017-12-14T00:00:00Z",
            3  => "2018-02-22T00:00:00Z",
            4  => "2018-05-01T00:00:00Z",
            5  => "2018-07-12T00:00:00Z",
            6  => "2018-09-27T00:00:00Z",
            7  => "2018-12-06T00:00:00Z",
            8  => "2019-02-28T00:00:00Z",
            9  => "2019-05-09T00:00:00Z",
            10 => "2019-08-01T00:00:00Z",
            11 => "2019-10-15T00:00:00Z",
            12 => "2020-02-20T00:00:00Z",
            13 => "2020-06-17T00:00:00Z",
            14 => "2020-08-27T00:00:00Z",
            15 => "2020-12-02T00:00:00Z",
            16 => "2021-03-16T00:00:00Z",
            17 => "2021-06-08T00:00:00Z",
            18 => "2021-09-13T00:00:00Z",
            19 => "2021-12-05T00:00:00Z",
            _  => "2020-01-01T00:00:00Z",
        }.to_string();

        // Parse game servers: "ip:port:playlist,ip:port:playlist"
        let game_servers: Vec<String> = env::var("GAME_SERVERS")
            .unwrap_or_else(|_| "127.0.0.1:7777:playlist_defaultsolo".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Config {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3551".into())
                .parse()
                .expect("PORT must be a valid number"),
            season_number,
            season_template_id,
            season_begin,
            season_end: "9999-01-01T00:00:00Z".to_string(),
            season_displayed_end: "9999-01-01T00:00:00Z".to_string(),
            matchmaker_ip: env::var("MATCHMAKER_IP")
                .unwrap_or_else(|_| "127.0.0.1:80".into()),
            game_servers,
        }
    }
}
