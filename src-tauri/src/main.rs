// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::get;

#[tauri::command]
async fn verify(lines: &str) -> Result<String, ()> {
    // 改行コードで分割
    let lines: Vec<&str> = lines.split("\n").collect();

    // 並列処理
    let mut tasks = Vec::new();
    for line in lines {
        tasks.push(judge_doi_existence(line));
    }

    // tasksの結果を待つ
    let mut results = Vec::new();
    for task in tasks {
        results.push(task.await);
    }

    // 結果を改行コードで結合
    let mut result = String::from("");
    for line in results {
        result += &line;
        result += "\n";
    }
    Ok(result)
}

async fn judge_doi_existence(line: &str) -> String {
    let url = parse_line(line);
    let mut result = String::from("");

    // exist_doiがエラーを返した場合はNG、そうでなければOKを返す
    match exist_doi(url).await {
        Ok(exist) => {
            if exist {
                result += "[OK]";
            } else {
                result += "[NG]";
            }
        }
        Err(_) => {
            result += "[NG]";
        }
    }
    result += line;
    result    
}

fn parse_line(line: &str) -> &str {
    let mut url = "";
    for word in line.split_whitespace() {
        if word.starts_with("https://doi.org/") || word.starts_with("http://dx.doi.org") {
            url = word;
        }
    }
    url
}

async fn exist_doi(url: &str) -> reqwest::Result<bool> {
    let body = get(url).await?;
    return Ok(!(body.status() == 404));
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, exist_doi};
    // use super::*;
    use mockito::mock;
    use tokio::runtime::Runtime;

    #[test]
    fn test_parse_url() {
        let line = "Chen, C., Lee, S.-Y., & Stevenson, H. W. (1995). Response style and cross-cultural comparisons of rating scales among East Asian and North American students. Psychological Science, 6(3), 170–175. https://doi.org/10.1111/j.1467-9280.1995.tb00327.x";
        assert_eq!(parse_line(line), "https://doi.org/10.1111/j.1467-9280.1995.tb00327.x");
    }

    #[test]
    fn test_exist_doi() {
        let _m = mock("GET", "/")
            .with_status(200)
            .with_body("doi exists")
            .create();

        let url = &mockito::server_url();
        let rt = Runtime::new().unwrap();

        let exist = rt.block_on(exist_doi(url)).unwrap();

        assert_eq!(exist, true);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
