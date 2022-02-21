use crate::gh::{
    ListModuleResponse, ListModuleResponseRefs, ListModuleResponseRefsPageInfo,
    ListModuleResponseReleases, ListModulesResponse, ListModulesResponsePageInfo,
};
use prettytable::{color, Attr, Cell, Row, Table};

fn add_modules_header(table: &mut Table, no_color: bool, description: bool, url: bool) {
    let use_color = !no_color;

    let name_header_value = "Name";
    let name_header = if use_color {
        Cell::new(name_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(name_header_value).with_style(Attr::Bold)
    };

    let latest_tag_header_value = "Latest Tag";
    let latest_tag_header = if use_color {
        Cell::new(latest_tag_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(latest_tag_header_value).with_style(Attr::Bold)
    };

    let latest_release_header_value = "Latest Release";
    let latest_release_header = if use_color {
        Cell::new(latest_release_header_value)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        Cell::new(latest_release_header_value).with_style(Attr::Bold)
    };

    let mut title_vec = vec![name_header];
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
    title_vec.push(latest_tag_header);
    title_vec.push(latest_release_header);

    table.set_titles(Row::new(title_vec));
}

fn add_modules_footer(
    table: &mut Table,
    total_count: u64,
    filtered_repos: u64,
    page_info: &ListModulesResponsePageInfo,
    no_color: bool,
    description: bool,
    url: bool,
) {
    let use_color = !no_color;

    let total_count_text = format!("{}", total_count);
    let total_count_cell = Cell::new(&total_count_text);

    let mut page_info_table = Table::new();

    let mut page_info_titles_vec = vec![Cell::new("Search Total")];
    if filtered_repos > 0 {
        page_info_titles_vec.push(Cell::new("Filtered Repos"));
    }
    if page_info.has_next_page {
        page_info_titles_vec.push(Cell::new("End Cursor"));
    }
    page_info_table.set_titles(Row::new(page_info_titles_vec));

    let mut page_info_vec = vec![total_count_cell];
    if filtered_repos > 0 {
        let filtered_repo_text = format!("{}", filtered_repos);
        let filtered_repo_cell = Cell::new(&filtered_repo_text);
        page_info_vec.push(filtered_repo_cell);
    }
    if page_info.has_next_page {
        let end_cursor = &page_info.end_cursor.clone().unwrap_or_default();
        let end_cursor_cell = Cell::new(&end_cursor);
        page_info_vec.push(end_cursor_cell);
    }

    page_info_table.add_row(Row::new(page_info_vec));

    let page_info_cell = if use_color {
        Cell::new(&page_info_table.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN))
    } else {
        Cell::new(&page_info_table.to_string()).with_style(Attr::Bold)
    };

    let mut footer_vec = vec![page_info_cell];

    if description {
        footer_vec.push(Cell::new(""));
    }

    if url {
        footer_vec.push(Cell::new(""));
    }

    table.add_row(Row::new(footer_vec));
}

pub fn print_modules_table(
    list_modules_response: ListModulesResponse,
    no_color: bool,
    description: bool,
    url: bool,
) {
    let mut table = Table::new();
    add_modules_header(&mut table, no_color, description, url);
    for module in list_modules_response.data.search.nodes {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&module.name));
        if description {
            row.add_cell(Cell::new(&module.description.unwrap_or_default()));
        }
        if url {
            row.add_cell(Cell::new(&module.url));
        }
        let latest_tag_name = if module.refs.nodes.is_empty() {
            "".to_string()
        } else {
            module.refs.nodes[0].name.clone()
        };
        row.add_cell(Cell::new(&latest_tag_name));
        let latest_release_name = if module.releases.nodes.is_empty() {
            "".to_string()
        } else {
            module.releases.nodes[0].name.clone()
        };
        row.add_cell(Cell::new(&latest_release_name));
        table.add_row(row);
    }
    add_modules_footer(
        &mut table,
        list_modules_response.data.search.repository_count,
        list_modules_response
            .data
            .search
            .filtered_repository_count
            .unwrap_or(0),
        &list_modules_response.data.search.page_info,
        no_color,
        description,
        url,
    );
    table.printstd();
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

    let mut title_vec = vec![name_header];
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

    let name_header_value = "Name";
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

fn add_tags_footer(
    table: &mut Table,
    total_count: u64,
    page_info: &ListModuleResponseRefsPageInfo,
    no_color: bool,
    url: bool,
) {
    let use_color = !no_color;

    let total_count_text = format!("{}", total_count);
    let total_count_cell = Cell::new(&total_count_text);

    let mut page_info_table = Table::new();

    let mut page_info_titles_vec = vec![Cell::new("Tags Total")];

    if page_info.has_next_page {
        page_info_titles_vec.push(Cell::new("End Cursor"));
    }

    page_info_table.set_titles(Row::new(page_info_titles_vec));

    let mut page_info_vec = vec![total_count_cell];

    if page_info.has_next_page {
        let end_cursor = &page_info.end_cursor.clone().unwrap_or_default();
        let end_cursor_cell = Cell::new(&end_cursor);
        page_info_vec.push(end_cursor_cell);
    }

    page_info_table.add_row(Row::new(page_info_vec));

    let page_info_cell = if use_color {
        Cell::new(&page_info_table.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN))
    } else {
        Cell::new(&page_info_table.to_string()).with_style(Attr::Bold)
    };

    let mut footer_vec = vec![page_info_cell];

    if url {
        footer_vec.push(Cell::new(""));
    }

    table.add_row(Row::new(footer_vec));
}

fn print_tags_table(tags: ListModuleResponseRefs, no_color: bool, url: bool) {
    let mut table = Table::new();
    add_tags_header(&mut table, no_color, url);
    for tag in tags.edges {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&tag.node.name));
        if url {
            let url_cell = Cell::new(&tag.node.target.commit_url);
            row.add_cell(url_cell);
        }
        table.add_row(row);
    }
    add_tags_footer(&mut table, tags.total_count, &tags.page_info, no_color, url);
    table.printstd();
}

pub fn print_module_table(
    list_module_response: ListModuleResponse,
    no_color: bool,
    description: bool,
    url: bool,
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
    let mut module_vec = vec![Cell::new(&list_module_response.data.repository.name)];
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
    if !tags_is_empty {
        print_tags_table(list_module_response.data.repository.refs, no_color, url);
    }
}
