use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::process::Command;

pub fn get_logged_in_user() -> String {
    let logged_in_user = Command::new("gh")
        .args(&["api", "user", "--jq", ".login"])
        .output()
        .expect("Could get logged in user");

    String::from_utf8(logged_in_user.stdout)
        .expect("Could not parse logged in user")
        .trim()
        .to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleItem {
    id: u64,
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponse {
    pub total_count: u64,
    incomplete_results: bool,
    pub items: Vec<ListModuleItem>,
}

pub fn list_modules(org: String) -> ListModuleResponse {
    let per_page = format!("per_page={}", 30);
    let page = format!("page={}", 1);
    let query = format!("q=terraform-module in:name user:{}", org);
    let modules = Command::new("gh")
        .args(&[
            "api",
            "-X",
            "GET",
            "search/repositories",
            "-f",
            &per_page,
            "-f",
            &page,
            "-f",
            &query,
        ])
        .output()
        .expect("Could not list modules");
    let stdout = modules.stdout;
    let listed_modules_output = String::from_utf8(stdout)
        .expect("Could not parse modules")
        .trim()
        .to_string();
    let mut dirty_list_module_response: ListModuleResponse =
        serde_json::from_str(&listed_modules_output).expect("Could not parse modules");
    let re = Regex::new(r"terraform-.*-module").unwrap();
    dirty_list_module_response
        .items
        .retain(|item| re.is_match(item.name.as_str()));
    dirty_list_module_response
}
