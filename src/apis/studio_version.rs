pub async fn get_studio_version() -> Result<String, reqwest::Error> {
    let response = reqwest::get(STUDIO_VERSION_URL).await?;
    response.text().await
}

const STUDIO_VERSION_URL: &str = "https://s3.amazonaws.com/setup.roblox.com/versionQTStudio";
