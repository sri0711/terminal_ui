
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchSongMain {
    pub results: Option<Vec<SearchSongResult>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchSongResult {
    pub id: Option<String>,
    pub image: Option<String>,
    pub language: Option<String>,
    pub perma_url: Option<String>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    pub year: Option<String>,
    pub more_info: Option<SearchSongMoreInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchSongMoreInfo {
    #[serde(deserialize_with = "convert_string")]
    pub encrypted_media_url: Option<String>,
    #[serde(rename = "320kbps")]
    pub is_320: Option<String>,
    pub vlink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_quality_link: Option<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
}

fn convert_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    Ok(Some(parse_url(value.expect("null")).to_owned()))
}
