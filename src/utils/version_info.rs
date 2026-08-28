
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub season: u32,
    pub build: f64,
    pub cl: String,
    pub lobby: String,
}

impl Default for VersionInfo {
    fn default() -> Self {
        VersionInfo {
            season: 0,
            build: 0.0,
            cl: "0".to_string(),
            lobby: "LobbySeason0".to_string(),
        }
    }
}

impl VersionInfo {
    /// Parse version info from the User-Agent header string.
    /// Example UA: `Fortnite/++Fortnite+Release-12.41-CL-11883027`
    pub fn from_user_agent(ua: &str) -> Self {
        let mut info = VersionInfo::default();

        // Extract CL (changelist)
        let cl = Self::extract_cl(ua);
        info.cl = cl.clone();

        // Try to extract build from "Release-X.Y" pattern
        if let Some(build_str) = Self::extract_build(ua) {
            if let Ok(build) = build_str.parse::<f64>() {
                info.build = build;
                info.season = build.floor() as u32;
                info.lobby = format!("LobbySeason{}", info.season);
                return info;
            }
        }

        // Fallback: use CL number to determine season
        if let Ok(cl_num) = cl.parse::<u64>() {
            if cl_num < 3_724_489 {
                info.season = 0;
                info.build = 0.0;
                info.lobby = "LobbySeason0".to_string();
            } else if cl_num <= 3_790_078 {
                info.season = 1;
                info.build = 1.0;
                info.lobby = "LobbySeason1".to_string();
            } else {
                info.season = 2;
                info.build = 2.0;
                info.lobby = "LobbyWinterDecor".to_string();
            }
        }

        info
    }

    fn extract_cl(ua: &str) -> String {
        // Try pattern: "Release-X.Y-CL-NNNNNN" split by "-" index 3
        let parts: Vec<&str> = ua.split('-').collect();
        if parts.len() > 3 {
            let candidate = parts[3].split(',').next().unwrap_or("").trim();
            if candidate.parse::<u64>().is_ok() {
                return candidate.to_string();
            }
            let candidate2 = parts[3].split(' ').next().unwrap_or("").trim();
            if candidate2.parse::<u64>().is_ok() {
                return candidate2.to_string();
            }
        }
        // Fallback: split by "-" index 1
        if parts.len() > 1 {
            let candidate = parts[1].split('+').next().unwrap_or("").trim();
            if candidate.parse::<u64>().is_ok() {
                return candidate.to_string();
            }
        }
        "0".to_string()
    }

    fn extract_build(ua: &str) -> Option<String> {
        // Find "Release-" prefix
        let release_idx = ua.find("Release-")?;
        let after_release = &ua[release_idx + 8..];
        let build_part = after_release.split('-').next()?;

        // Handle 3-segment versions like "12.41.0" -> "12.410"
        let segments: Vec<&str> = build_part.split('.').collect();
        if segments.len() == 3 {
            Some(format!("{}.{}{}", segments[0], segments[1], segments[2]))
        } else {
            Some(build_part.to_string())
        }
    }
}
