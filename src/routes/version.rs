use axum::{Router, routing::get, response::IntoResponse, Extension};
use axum::http::HeaderMap;
use chrono::Utc;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::config::Config;
use crate::utils::version_info::VersionInfo;

pub fn router() -> Router {
    Router::new()
        .route("/fortnite/api/v2/versioncheck", get(version_check))
        .route("/fortnite/api/v2/versioncheck/:version", get(version_check))
        .route("/fortnite/api/calendar/v1/timeline", get(timeline))
}

async fn version_check() -> impl IntoResponse {
    axum::Json(json!({ "type": "NO_UPDATE" }))
}

async fn timeline(
    headers: HeaderMap,
    Extension(config): Extension<Arc<Config>>,
) -> impl IntoResponse {
    let ua = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let memory = if ua.is_empty() {
        VersionInfo {
            season: config.season_number.floor() as u32,
            build: config.season_number,
            cl: "0".to_string(),
            lobby: format!("LobbySeason{}", config.season_number.floor() as u32),
        }
    } else {
        VersionInfo::from_user_agent(ua)
    };

    let current_time = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    // Build activeEvents based on season/build — mirrors timeline.js from Better-Reload
    let mut active_events: Vec<Value> = vec![
        json!({
            "eventType": format!("EventFlag.Season{}", memory.season),
            "activeUntil": "9999-01-01T00:00:00.000Z",
            "activeSince": "2020-01-01T00:00:00.000Z"
        }),
        json!({
            "eventType": format!("EventFlag.{}", memory.lobby),
            "activeUntil": "9999-01-01T00:00:00.000Z",
            "activeSince": "2020-01-01T00:00:00.000Z"
        }),
    ];

    // Season-specific events
    match memory.season {
        3 => {
            active_events.push(json!({ "eventType": "EventFlag.Spring2018Phase1", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            if memory.build >= 3.1 { active_events.push(json!({ "eventType": "EventFlag.Spring2018Phase2", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
            if memory.build >= 3.3 { active_events.push(json!({ "eventType": "EventFlag.Spring2018Phase3", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
            if memory.build >= 3.4 { active_events.push(json!({ "eventType": "EventFlag.Spring2018Phase4", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
        }
        4 => {
            active_events.push(json!({ "eventType": "EventFlag.Blockbuster2018", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            active_events.push(json!({ "eventType": "EventFlag.Blockbuster2018Phase1", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            if memory.build >= 4.3 { active_events.push(json!({ "eventType": "EventFlag.Blockbuster2018Phase2", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
            if memory.build >= 4.4 { active_events.push(json!({ "eventType": "EventFlag.Blockbuster2018Phase3", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
            if memory.build >= 4.5 { active_events.push(json!({ "eventType": "EventFlag.Blockbuster2018Phase4", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
        }
        5 => {
            for event in &["EventFlag.RoadTrip2018", "EventFlag.Horde", "EventFlag.Anniversary2018_BR", "EventFlag.LTM_Heist"] {
                active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            }
        }
        6 => {
            active_events.push(json!({ "eventType": "EventFlag.LTM_Fortnitemares", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            active_events.push(json!({ "eventType": "EventFlag.LTM_LilKevin", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            if memory.build >= 6.20 {
                for event in &["EventFlag.Fortnitemares", "EventFlag.FortnitemaresPhase1", "POI0"] {
                    active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
                }
            }
            if memory.build >= 6.22 { active_events.push(json!({ "eventType": "EventFlag.FortnitemaresPhase2", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
        }
        7 => {
            for event in &["EventFlag.Frostnite", "EventFlag.LTM_14DaysOfFortnite", "EventFlag.LTE_Festivus", "EventFlag.LTM_WinterDeimos", "EventFlag.LTE_S7_OverTime"] {
                active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            }
        }
        8 => {
            for event in &["EventFlag.Spring2019", "EventFlag.Spring2019.Phase1", "EventFlag.LTM_Ashton", "EventFlag.LTM_Goose", "EventFlag.LTM_HighStakes", "EventFlag.LTE_BootyBay"] {
                active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            }
            if memory.build >= 8.2 { active_events.push(json!({ "eventType": "EventFlag.Spring2019.Phase2", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
        }
        9 => {
            for event in &["EventFlag.Season9.Phase1", "EventFlag.Anniversary2019_BR", "EventFlag.LTM_14DaysOfSummer", "EventFlag.LTM_Mash", "EventFlag.LTM_Wax"] {
                active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            }
            if memory.build >= 9.2 { active_events.push(json!({ "eventType": "EventFlag.Season9.Phase2", "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" })); }
        }
        10 => {
            for event in &["EventFlag.Mayday", "EventFlag.Season10.Phase2", "EventFlag.Season10.Phase3", "EventFlag.LTE_BlackMonday", "EventFlag.S10_Oak", "EventFlag.S10_Mystery"] {
                active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            }
        }
        11 => {
            for event in &["EventFlag.LTE_CoinCollectXP", "EventFlag.LTE_Fortnitemares2019", "EventFlag.LTE_Galileo_Feats", "EventFlag.LTE_Galileo", "EventFlag.LTE_WinterFest2019"] {
                active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            }
        }
        12 => {
            for event in &["EventFlag.LTE_SpyGames", "EventFlag.LTE_JerkyChallenges", "EventFlag.LTE_Oro", "EventFlag.LTE_StormTheAgency"] {
                active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            }
        }
        19 => {
            for event in &["EventFlag.LTM_Hyena", "EventFlag.LTM_Vigilante", "EventFlag.LTM_ZebraWallet", "EventFlag.LTE_Galileo_Feats", "EventFlag.Event_S19_Trey"] {
                active_events.push(json!({ "eventType": event, "activeUntil": "9999-01-01T00:00:00.000Z", "activeSince": "2020-01-01T00:00:00.000Z" }));
            }
        }
        _ => {}
    }

    // Build state template using config or parsed season
    let season_num = if memory.season > 0 { memory.season } else { config.season_number.floor() as u32 };
    let season_template_id = format!("AthenaSeason:athenaseason{}", season_num);

    let state_template = json!({
        "activeStorefronts": [],
        "eventNamedWeights": {},
        "seasonNumber": season_num,
        "seasonTemplateId": season_template_id,
        "matchXpBonusPoints": 0,
        "seasonBegin": config.season_begin,
        "seasonEnd": config.season_end,
        "seasonDisplayedEnd": config.season_displayed_end,
        "weeklyStoreEnd": "9999-01-01T00:00:00Z",
        "stwEventStoreEnd": "9999-01-01T00:00:00.000Z",
        "stwWeeklyStoreEnd": "9999-01-01T00:00:00.000Z",
        "sectionStoreEnds": { "Featured": "9999-01-01T00:00:00.000Z" },
        "dailyStoreEnd": "9999-01-01T00:00:00Z"
    });

    axum::Json(json!({
        "channels": {
            "client-matchmaking": {
                "states": [],
                "cacheExpire": "9999-01-01T00:00:00.000Z"
            },
            "client-events": {
                "states": [{
                    "validFrom": "0001-01-01T00:00:00.000Z",
                    "activeEvents": active_events,
                    "state": state_template
                }],
                "cacheExpire": "9999-01-01T00:00:00.000Z"
            }
        },
        "eventsTimeOffsetHrs": 0,
        "cacheIntervalMins": 10,
        "currentTime": current_time
    }))
}
