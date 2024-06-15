use crate::models::match_model::Match;

pub async fn get_matches() -> Result<Vec<Match>, reqwest::Error> {
    let response = reqwest::get("http://localhost:8080/matches".to_string()).await?;
    let matches = response.json::<Vec<Match>>().await?;
    for data in &matches {
        println!("{} {} {} {} {} {}",
                 data.id,
                 data.date,
                 data.first_team_name,
                 data.second_team_name,
                 data.first_team_score,
                 data.second_team_score);
    }
    Ok(matches)
}

pub async fn get_match(id: &str) -> Result<Match, reqwest::Error> {
    let response = reqwest::get(format!("http://localhost:8080/matches/{}", id)).await?;
    let data = response.json::<Match>().await?;
    println!("{} {} {} {} {} {}",
             data.id,
             data.date,
             data.first_team_name,
             data.second_team_name,
             data.first_team_score,
             data.second_team_score);
    Ok(data)
}