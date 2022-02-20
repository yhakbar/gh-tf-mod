use anyhow::Result;
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
pub struct ListModulesResponsePageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModulesResponseRelease {
    pub tag_name: String,
    pub published_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesResponseReleases {
    pub nodes: Vec<ListModulesResponseRelease>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesResponseRef {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesResponseRefs {
    pub nodes: Vec<ListModulesResponseRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesResponseNode {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub releases: ListModulesResponseReleases,
    pub refs: ListModulesResponseRefs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModulesResponseSearch {
    pub page_info: ListModulesResponsePageInfo,
    pub repository_count: u64,
    pub filtered_repository_count: Option<u64>,
    pub nodes: Vec<ListModulesResponseNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesResponseData {
    pub search: ListModulesResponseSearch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesResponse {
    pub data: ListModulesResponseData,
}

pub fn list_modules(
    org: String,
    provider: Option<String>,
    first: Option<usize>,
    after: Option<String>,
) -> Result<ListModulesResponse, std::io::Error> {
    let query_provider = match provider {
        Some(ref provider) => format!("{}-", provider),
        None => "".to_string(),
    };
    let query_first = if first.is_some() {
        format!("{}", first.unwrap())
    } else {
        "30".to_string()
    };
    let query_after = if after.is_some() {
        format!("\"{}\"", after.unwrap())
    } else {
        "null".to_string()
    };
    let query = format!("query {{
        search(query: \"terraform-{}module in:name user:{}\", type: REPOSITORY, first: {}, after: {}) {{
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
    }}", query_provider, org, query_first, query_after);

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
        let mut list_module_response: ListModulesResponse =
            serde_json::from_str(&listed_modules_output).expect("Could not parse modules");
        let regex_provider = if provider.is_some() {
            format!("{}-", &provider.unwrap())
        } else {
            "".to_string()
        };
        let regex_pattern = format!("terraform-{}.*-module", regex_provider);
        let re = Regex::new(&regex_pattern).unwrap();
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseReleaseNode {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseRefNode {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseRelease {
    pub node: ListModuleResponseReleaseNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseRef {
    pub node: ListModuleResponseRefNode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModuleResponseReleasesPageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModuleResponseRefsPageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModuleResponseReleases {
    pub edges: Vec<ListModuleResponseRelease>,
    pub page_info: ListModuleResponseReleasesPageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModuleResponseRefs {
    pub edges: Vec<ListModuleResponseRef>,
    pub page_info: ListModuleResponseRefsPageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseRepository {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub releases: ListModuleResponseReleases,
    pub refs: ListModuleResponseRefs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponseData {
    pub repository: ListModuleResponseRepository,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModuleResponse {
    pub data: ListModuleResponseData,
}

pub fn list_module(
    org: String,
    provider: Option<String>,
    module: String,
    first: Option<usize>,
    after: Option<String>,
) -> Result<ListModuleResponse, std::io::Error> {
    let query_provider = match provider {
        Some(ref provider) => format!("{}-", provider),
        None => "".to_string(),
    };

    let query_module = format!("terraform-{}{}-module", query_provider, module);

    let query_first = if first.is_some() {
        format!("{}", first.unwrap())
    } else {
        "30".to_string()
    };
    let query_after = if after.is_some() {
        format!("\"{}\"", after.unwrap())
    } else {
        "null".to_string()
    };

    let query = format!(
        "{{
        repository(name: \"{}\", owner: \"{}\") {{
            name
            description
            url
            releases(first: {}, after: {}, orderBy: {{field: CREATED_AT, direction: DESC}}) {{
                edges {{
                    node {{
                        name
                    }}
                }}
                pageInfo {{
                    endCursor
                    hasNextPage
                    startCursor
                    hasPreviousPage
                }}
            }}
            refs(refPrefix: \"refs/tags/\", first: {}, after: {}, orderBy: {{field: TAG_COMMIT_DATE, direction: DESC}}) {{
                edges {{
                    node {{
                        name
                    }}
                }}
                pageInfo {{
                    endCursor
                    hasNextPage
                    startCursor
                    hasPreviousPage
                }}
            }}
        }}
    }}",
        query_module, org, query_first, query_after, query_first, query_after
    );

    let query_parameter = format!("query={}", &query);

    let module = Command::new("gh")
        .args(&["api", "graphql", "-f", &query_parameter])
        .output()
        .expect("Could not list module");

    let stdout = module.stdout;

    let listed_module_output = String::from_utf8(stdout)
        .expect("Could not parse module")
        .trim()
        .to_string();

    if module.status.code() == Some(0) {
        let list_module_response: ListModuleResponse =
            serde_json::from_str(&listed_module_output).expect("Could not parse module");
        Ok(list_module_response)
    } else {
        let stderr = module.stderr;
        let listed_module_stderr = String::from_utf8(stderr)
            .expect("Could not parse module")
            .trim()
            .to_string();
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            listed_module_stderr,
        ))
    }
}
