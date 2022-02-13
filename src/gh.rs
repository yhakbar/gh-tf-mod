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
#[serde(rename_all = "camelCase")]
pub struct ListModuleResponsePageInfo {
    pub has_next_page: bool,
    pub end_cursor: String,
    pub has_previous_page: bool,
    pub start_cursor: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModuleResponseRelease {
    pub tag_name: String,
    pub published_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseReleases {
    pub nodes: Vec<ListModuleResponseRelease>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseRef {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesResponseRefs {
    pub nodes: Vec<ListModuleResponseRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseNode {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub releases: ListModuleResponseReleases,
    pub refs: ListModulesResponseRefs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModuleResponseSearch {
    pub page_info: ListModuleResponsePageInfo,
    pub repository_count: u64,
    pub filtered_repository_count: Option<u64>,
    pub nodes: Vec<ListModuleResponseNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseData {
    pub search: ListModuleResponseSearch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponse {
    pub data: ListModuleResponseData,
}

pub fn list_modules(org: String) -> Result<ListModuleResponse, std::io::Error> {
    let query = format!("query {{
        search(query: \"terraform-module in:name user:{}\", type: REPOSITORY, first: 30, after: null) {{
            pageInfo {{
                hasNextPage
                endCursor
                hasPreviousPage
                startCursor
            }}
            repositoryCount
            nodes {{
                ... on Repository {{
                    name
                    description
                    url
                    releases(last: 1) {{
                        nodes {{
                            tagName
                            publishedAt
                        }}
                    }}
                    refs(refPrefix: \"refs/tags/\", last: 1) {{
                        nodes {{
                            name
                        }}
                    }}
                }}
            }}
        }}
    }}", org);

    let query_parameter = format!("query={}", &query);

    let modules = Command::new("gh")
        .args(&["api", "graphql", "-f", &query_parameter])
        .output()
        .expect("Could not list modules");

    let stdout = modules.stdout;

    let listed_modules_output = String::from_utf8(stdout)
        .expect("Could not parse modules")
        .trim()
        .to_string();

    if modules.status.code() == Some(0) {
        let mut list_module_response: ListModuleResponse =
            serde_json::from_str(&listed_modules_output).expect("Could not parse modules");
        let re = Regex::new(r"terraform-.*-module").unwrap();
        let pre_sift_len = list_module_response.data.search.nodes.len() as u64;
        list_module_response
            .data
            .search
            .nodes
            .retain(|item| re.is_match(item.name.as_str()));
        let post_sift_len = list_module_response.data.search.nodes.len() as u64;
        list_module_response.data.search.filtered_repository_count =
            Some(pre_sift_len - post_sift_len);
        Ok(list_module_response)
    } else {
        println!("Failed to list modules:");
        let stderr = modules.stderr;
        let listed_modules_stderr = String::from_utf8(stderr)
            .expect("Could not parse modules")
            .trim()
            .to_string();
        panic!("{}", listed_modules_stderr);
    }
}
