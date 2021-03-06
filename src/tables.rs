use crate::gh::{
    ListModuleResponse, ListModuleResponseRefs, ListModuleResponseRefsPageInfo,
    ListModuleResponseReleases, ListModuleResponseReleasesPageInfo, ListModulesResponse,
    ListModulesResponsePageInfo,
};
use prettytable::{color, Attr, Cell, Row, Table};

fn add_modules_header(
    table: &mut Table,
    no_color: bool,
    description: bool,
    url: bool,
    tags: bool,
    releases: bool,
) {
    let use_color = !no_color;

    let name_header_value = "Name";
    let name_header = if use_color {
        Cell::new(name_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(name_header_value).with_style(Attr::Bold)
    };

    let provider_header_value = "Provider";
    let provider_header = if use_color {
        Cell::new(provider_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(provider_header_value).with_style(Attr::Bold)
    };

    let mut title_vec = vec![name_header, provider_header];
    if description {
        let description_header_value = "Description";
        let description_header = if use_color {
            Cell::new(description_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(description_header_value).with_style(Attr::Bold)
        };
        title_vec.push(description_header);
    }
    if url {
        let url_header_value = "URL";
        let url_header = if use_color {
            Cell::new(url_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(url_header_value).with_style(Attr::Bold)
        };
        title_vec.push(url_header);
    }
    if tags {
        let latest_tag_header_value = "Latest Tag";
        let latest_tag_header = if use_color {
            Cell::new(latest_tag_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(latest_tag_header_value).with_style(Attr::Bold)
        };
        title_vec.push(latest_tag_header);
    }
    if releases {
        let latest_release_header_value = "Latest Release";
        let latest_release_header = if use_color {
            Cell::new(latest_release_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(latest_release_header_value).with_style(Attr::Bold)
        };
        title_vec.push(latest_release_header);
    }

    table.set_titles(Row::new(title_vec));
}

fn print_modules_paging_info(
    total_count: u64,
    filtered_repos: u64,
    page_info: &ListModulesResponsePageInfo,
    no_color: bool,
) {
    let use_color = !no_color;

    let total_count_text = format!("{}", total_count);
    let total_count_cell = Cell::new(&total_count_text);

    let mut page_info_table = Table::new();

    let mut page_info_titles_vec = vec![];

    let repos_header_value = "Repos";
    let repos_header = if use_color {
        Cell::new(repos_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(repos_header_value).with_style(Attr::Bold)
    };
    page_info_titles_vec.push(repos_header);

    if filtered_repos > 0 {
        let filtered_repos_header_value = "Hidden Repos";
        let filtered_repos_header = if use_color {
            Cell::new(filtered_repos_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(filtered_repos_header_value).with_style(Attr::Bold)
        };
        page_info_titles_vec.push(filtered_repos_header);
    }
    if page_info.has_next_page {
        let end_cursor_header_value = "End Cursor";
        let end_cursor_header = if use_color {
            Cell::new(end_cursor_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(end_cursor_header_value).with_style(Attr::Bold)
        };
        page_info_titles_vec.push(end_cursor_header);
    }
    page_info_table.set_titles(Row::new(page_info_titles_vec));

    let mut page_info_vec = vec![total_count_cell];
    if filtered_repos > 0 {
        let filtered_repo_text = format!("{}", filtered_repos);
        let filtered_repo_cell = if use_color {
            Cell::new(&filtered_repo_text).with_style(Attr::ForegroundColor(color::RED))
        } else {
            Cell::new(&filtered_repo_text)
        };
        page_info_vec.push(filtered_repo_cell);
    }
    if page_info.has_next_page {
        let end_cursor = &page_info.end_cursor.clone().unwrap_or_default();
        let end_cursor_cell = if use_color {
            Cell::new(&end_cursor).with_style(Attr::ForegroundColor(color::GREEN))
        } else {
            Cell::new(&end_cursor)
        };
        page_info_vec.push(end_cursor_cell);
    }

    page_info_table.add_row(Row::new(page_info_vec));
    page_info_table.printstd();
}

pub fn print_modules_table(
    list_modules_response: ListModulesResponse,
    no_color: bool,
    description: bool,
    url: bool,
    tags: bool,
    releases: bool,
) {
    let use_color = !no_color;

    let mut table = Table::new();
    add_modules_header(&mut table, no_color, description, url, tags, releases);
    for module in list_modules_response.data.search.nodes {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&module.short_name.unwrap_or(module.name)));
        row.add_cell(Cell::new(&module.provider.unwrap_or_default()));
        if description {
            row.add_cell(Cell::new(&module.description.unwrap_or_default()));
        }
        if url {
            let url_cell = if use_color {
                Cell::new(&module.url).with_style(Attr::ForegroundColor(color::BLUE))
            } else {
                Cell::new(&module.url)
            };
            row.add_cell(url_cell);
        }
        if tags {
            let latest_tag_name = if module.refs.nodes.is_empty() {
                "".to_string()
            } else {
                module.refs.nodes[0].name.clone()
            };
            row.add_cell(Cell::new(&latest_tag_name));
        }
        if releases {
            let latest_release_name = if module.releases.nodes.is_empty() {
                "".to_string()
            } else {
                module.releases.nodes[0].name.clone()
            };
            row.add_cell(Cell::new(&latest_release_name));
        }
        table.add_row(row);
    }
    table.printstd();
    print_modules_paging_info(
        list_modules_response.data.search.repository_count,
        list_modules_response
            .data
            .search
            .filtered_repository_count
            .unwrap_or(0),
        &list_modules_response.data.search.page_info,
        no_color,
    );
}

fn add_module_header(
    table: &mut Table,
    no_color: bool,
    description: bool,
    url: bool,
    tags_is_empty: bool,
    releases_is_empty: bool,
) {
    let use_color = !no_color;

    let name_header_value = "Name";
    let name_header = if use_color {
        Cell::new(name_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(name_header_value).with_style(Attr::Bold)
    };

    let provider_header_value = "Provider";
    let provider_header = if use_color {
        Cell::new(provider_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(provider_header_value).with_style(Attr::Bold)
    };

    let mut title_vec = vec![name_header, provider_header];
    if description {
        let description_header_value = "Description";
        let description_header = if use_color {
            Cell::new(description_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(description_header_value).with_style(Attr::Bold)
        };
        title_vec.push(description_header);
    }
    if url {
        let url_header_value = "URL";
        let url_header = if use_color {
            Cell::new(url_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(url_header_value).with_style(Attr::Bold)
        };
        title_vec.push(url_header);
    }
    if !tags_is_empty {
        let latest_tag_header_value = "Latest Tag";
        let latest_tag_header = if use_color {
            Cell::new(latest_tag_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(latest_tag_header_value).with_style(Attr::Bold)
        };
        title_vec.push(latest_tag_header);
    }
    if !releases_is_empty {
        let latest_release_header_value = "Latest Release";
        let latest_release_header = if use_color {
            Cell::new(latest_release_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(latest_release_header_value).with_style(Attr::Bold)
        };
        title_vec.push(latest_release_header);
    }

    table.set_titles(Row::new(title_vec));
}

fn add_tags_header(table: &mut Table, no_color: bool, url: bool) {
    let use_color = !no_color;

    let name_header_value = "Tag";
    let name_header = if use_color {
        Cell::new(name_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(name_header_value).with_style(Attr::Bold)
    };

    let mut title_vec = vec![name_header];
    if url {
        let url_header_value = "URL";
        let url_header = if use_color {
            Cell::new(url_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(url_header_value).with_style(Attr::Bold)
        };
        title_vec.push(url_header);
    }

    table.set_titles(Row::new(title_vec));
}

fn print_tags_paging_info(
    total_count: u64,
    page_info: &ListModuleResponseRefsPageInfo,
    no_color: bool,
) {
    let use_color = !no_color;

    let mut page_info_table = Table::new();

    let mut page_info_titles_vec = vec![];

    let tags_total_header_value = "Tags Total";
    let tags_total_header = if use_color {
        Cell::new(tags_total_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(tags_total_header_value).with_style(Attr::Bold)
    };
    page_info_titles_vec.push(tags_total_header);

    if page_info.has_next_page {
        let end_cursor_header_value = "End Cursor";
        let end_cursor_header = if use_color {
            Cell::new(end_cursor_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(end_cursor_header_value).with_style(Attr::Bold)
        };
        page_info_titles_vec.push(end_cursor_header);
    }

    page_info_table.set_titles(Row::new(page_info_titles_vec));

    let total_count_text = format!("{}", total_count);
    let total_count_cell = Cell::new(&total_count_text);
    let mut page_info_vec = vec![total_count_cell];

    if page_info.has_next_page {
        let end_cursor = &page_info.end_cursor.clone().unwrap_or_default();
        let end_cursor_cell = if use_color {
            Cell::new(end_cursor).with_style(Attr::ForegroundColor(color::GREEN))
        } else {
            Cell::new(end_cursor)
        };
        page_info_vec.push(end_cursor_cell);
    }

    page_info_table.add_row(Row::new(page_info_vec));

    page_info_table.printstd();
}

fn print_tags_table(tags: ListModuleResponseRefs, no_color: bool, url: bool) {
    let use_color = !no_color;
    let mut table = Table::new();
    add_tags_header(&mut table, no_color, url);
    for tag in tags.edges {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&tag.node.name));
        if url {
            let url_cell = if use_color {
                Cell::new(&tag.node.target.commit_url)
                    .with_style(Attr::ForegroundColor(color::BLUE))
            } else {
                Cell::new(&tag.node.target.commit_url)
            };
            row.add_cell(url_cell);
        }
        table.add_row(row);
    }
    table.printstd();
    print_tags_paging_info(tags.total_count, &tags.page_info, no_color);
}

fn add_releases_header(table: &mut Table, no_color: bool, url: bool, tags: bool) {
    let use_color = !no_color;

    let name_header_value = "Release";
    let name_header = if use_color {
        Cell::new(name_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(name_header_value).with_style(Attr::Bold)
    };

    let mut title_vec = vec![name_header];
    if tags {
        let tags_header_value = "Tag";
        let tags_header = if use_color {
            Cell::new(tags_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(tags_header_value).with_style(Attr::Bold)
        };
        title_vec.push(tags_header);
    }
    if url {
        let url_header_value = "URL";
        let url_header = if use_color {
            Cell::new(url_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(url_header_value).with_style(Attr::Bold)
        };
        title_vec.push(url_header);
    }

    table.set_titles(Row::new(title_vec));
}

fn print_releases_paging_info(
    total_count: u64,
    page_info: &ListModuleResponseReleasesPageInfo,
    no_color: bool,
) {
    let use_color = !no_color;

    let mut page_info_table = Table::new();

    let mut page_info_titles_vec = vec![];

    let releases_total_header_value = "Releases Total";
    let releases_total_header = if use_color {
        Cell::new(releases_total_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(releases_total_header_value).with_style(Attr::Bold)
    };
    page_info_titles_vec.push(releases_total_header);

    if page_info.has_next_page {
        let end_cursor_header_value = "End Cursor";
        let end_cursor_header = if use_color {
            Cell::new(end_cursor_header_value)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        } else {
            Cell::new(end_cursor_header_value).with_style(Attr::Bold)
        };
        page_info_titles_vec.push(end_cursor_header);
    }

    page_info_table.set_titles(Row::new(page_info_titles_vec));

    let total_count_text = format!("{}", total_count);
    let total_count_cell = Cell::new(&total_count_text);
    let mut page_info_vec = vec![total_count_cell];

    if page_info.has_next_page {
        let end_cursor = &page_info.end_cursor.clone().unwrap_or_default();
        let end_cursor_cell = if use_color {
            Cell::new(end_cursor).with_style(Attr::ForegroundColor(color::GREEN))
        } else {
            Cell::new(end_cursor)
        };
        page_info_vec.push(end_cursor_cell);
    }

    page_info_table.add_row(Row::new(page_info_vec));

    page_info_table.printstd();
}

fn print_releases_table(
    releases: ListModuleResponseReleases,
    no_color: bool,
    url: bool,
    tags: bool,
) {
    let use_color = !no_color;
    let mut table = Table::new();
    add_releases_header(&mut table, no_color, url, tags);
    for release in releases.edges {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&release.node.name));
        if tags {
            let tags_cell = Cell::new(&release.node.tag.name);
            row.add_cell(tags_cell);
        }
        if url {
            let url_cell = if use_color {
                Cell::new(&release.node.url).with_style(Attr::ForegroundColor(color::BLUE))
            } else {
                Cell::new(&release.node.url)
            };
            row.add_cell(url_cell);
        }
        table.add_row(row);
    }
    table.printstd();
    print_releases_paging_info(releases.total_count, &releases.page_info, no_color);
}

pub fn print_module_table(
    list_module_response: ListModuleResponse,
    no_color: bool,
    description: bool,
    url: bool,
    tags: bool,
    releases: bool,
) {
    let mut table = Table::new();
    let tags_is_empty = list_module_response.data.repository.refs.edges.is_empty();
    let releases_is_empty = list_module_response
        .data
        .repository
        .releases
        .edges
        .is_empty();
    add_module_header(
        &mut table,
        no_color,
        description,
        url,
        tags_is_empty,
        releases_is_empty,
    );
    let mut module_vec = vec![
        Cell::new(
            &list_module_response
                .data
                .repository
                .short_name
                .unwrap_or_default(),
        ),
        Cell::new(
            &list_module_response
                .data
                .repository
                .provider
                .unwrap_or_default(),
        ),
    ];
    if description {
        module_vec.push(Cell::new(
            &list_module_response
                .data
                .repository
                .description
                .unwrap_or_default(),
        ));
    }
    if url {
        module_vec.push(Cell::new(&list_module_response.data.repository.url));
    }
    if !tags_is_empty {
        module_vec.push(Cell::new(
            &list_module_response.data.repository.refs.edges[0].node.name,
        ));
    }
    if !releases_is_empty {
        module_vec.push(Cell::new(
            &list_module_response.data.repository.releases.edges[0]
                .node
                .name,
        ));
    }
    table.add_row(Row::new(module_vec));
    table.printstd();
    if tags && !tags_is_empty {
        print_tags_table(list_module_response.data.repository.refs, no_color, url);
    }
    if releases && !releases_is_empty {
        print_releases_table(
            list_module_response.data.repository.releases,
            no_color,
            url,
            tags,
        );
    }
}
