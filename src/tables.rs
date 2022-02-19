use crate::gh::{ListModulesResponse, ListModulesResponsePageInfo};
use prettytable::{color, Attr, Cell, Row, Table};

fn add_header(table: &mut Table, no_color: bool) {
    let use_color = !no_color;

    let mut name_cell = Cell::new("Name").with_style(Attr::Bold);
    name_cell = if use_color {
        name_cell.with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        name_cell
    };

    let mut latest_tag_cell = Cell::new("Latest Tag").with_style(Attr::Bold);
    latest_tag_cell = if use_color {
        latest_tag_cell.with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        latest_tag_cell
    };

    let mut latest_release_cell = Cell::new("Latest Release").with_style(Attr::Bold);
    latest_release_cell = if use_color {
        latest_release_cell.with_style(Attr::ForegroundColor(color::CYAN))
    } else {
        latest_release_cell
    };

    table.set_titles(Row::new(vec![
        name_cell,
        // TODO Add flag for these
        // Cell::new("Description").with_style(Attr::Bold),
        // Cell::new("URL").with_style(Attr::Bold),
        latest_tag_cell,
        latest_release_cell,
    ]));
}

fn add_footer(
    table: &mut Table,
    total_count: u64,
    filtered_repos: u64,
    page_info: &ListModulesResponsePageInfo,
    no_color: bool,
) {
    let use_color = !no_color;

    let total_count_text = format!("{}", total_count);
    let mut total_count_cell = Cell::new(&total_count_text).with_style(Attr::Bold);
    total_count_cell = if use_color {
        total_count_cell.with_style(Attr::ForegroundColor(color::GREEN))
    } else {
        total_count_cell
    };

    let filtered_repo_text = format!("{}", filtered_repos);
    let mut filtered_repo_cell = Cell::new(&filtered_repo_text).with_style(Attr::Bold);
    filtered_repo_cell = if use_color {
        filtered_repo_cell.with_style(Attr::ForegroundColor(color::RED))
    } else {
        filtered_repo_cell
    };

    let mut paging_info_table = Table::new();
    paging_info_table.set_titles(Row::new(vec![
        Cell::new("Search Total"),
        Cell::new("Filtered"),
        Cell::new("Start Cursor"),
        Cell::new("End Cursor"),
    ]));

    let end_cursor = &page_info.end_cursor.clone().unwrap_or_default();
    let start_cursor_cell = if use_color {
        if page_info.has_previous_page {
            Cell::new(&end_cursor).with_style(Attr::ForegroundColor(color::GREEN))
        } else {
            Cell::new(&end_cursor).with_style(Attr::ForegroundColor(color::RED))
        }
    } else {
        Cell::new(&end_cursor)
    };

    let end_cursor = &page_info.end_cursor.clone().unwrap_or_default();
    let end_cursor_cell = if use_color {
        if page_info.has_next_page {
            Cell::new(&end_cursor).with_style(Attr::ForegroundColor(color::GREEN))
        } else {
            Cell::new(&end_cursor).with_style(Attr::ForegroundColor(color::RED))
        }
    } else {
        Cell::new(&end_cursor)
    };

    paging_info_table.add_row(Row::new(vec![
        total_count_cell,
        filtered_repo_cell,
        start_cursor_cell,
        end_cursor_cell,
    ]));

    let paging_info_cell = Cell::new(&paging_info_table.to_string());

    let has_previous_page_text = if page_info.has_previous_page {
        "\n\n← Has Previous Page"
    } else {
        "\n\nFirst Page"
    };
    let mut left_arrow_cell = Cell::new(&has_previous_page_text).with_style(Attr::Bold);
    if page_info.has_previous_page {
        left_arrow_cell = if use_color {
            left_arrow_cell.with_style(Attr::ForegroundColor(color::GREEN))
        } else {
            left_arrow_cell
        };
    } else {
        left_arrow_cell = if use_color {
            left_arrow_cell.with_style(Attr::ForegroundColor(color::RED))
        } else {
            left_arrow_cell
        };
    }

    let has_next_page_text = if page_info.has_next_page {
        "\n\nHas Next Page →"
    } else {
        "\n\nLast Page"
    };

    let mut right_arrow_cell = Cell::new(&has_next_page_text).with_style(Attr::Bold);

    if page_info.has_next_page {
        right_arrow_cell = if use_color {
            right_arrow_cell.with_style(Attr::ForegroundColor(color::GREEN))
        } else {
            right_arrow_cell
        };
    } else {
        right_arrow_cell = if use_color {
            right_arrow_cell.with_style(Attr::ForegroundColor(color::RED))
        } else {
            right_arrow_cell
        };
    }

    table.add_row(Row::new(vec![
        paging_info_cell,
        left_arrow_cell,
        right_arrow_cell,
    ]));
}

pub fn print_modules_table(list_modules_response: ListModulesResponse, no_color: bool) {
    let mut table = Table::new();
    add_header(&mut table, no_color);
    for module in list_modules_response.data.search.nodes {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&module.name));
        // TODO: Add flag for description
        // row.add_cell(Cell::new(&module.description.unwrap_or("".to_string())));
        // TODO: Add flag for url
        // row.add_cell(Cell::new(&module.url));
        let latest_tag_name = if module.refs.nodes.is_empty() {
            "".to_string()
        } else {
            module.refs.nodes[0].name.clone()
        };
        row.add_cell(Cell::new(&latest_tag_name));
        let latest_release_name = if module.releases.nodes.is_empty() {
            "".to_string()
        } else {
            module.releases.nodes[0].tag_name.clone()
        };
        row.add_cell(Cell::new(&latest_release_name));
        table.add_row(row);
    }
    add_footer(
        &mut table,
        list_modules_response.data.search.repository_count,
        list_modules_response
            .data
            .search
            .filtered_repository_count
            .unwrap_or(0),
        &list_modules_response.data.search.page_info,
        no_color,
    );
    table.printstd();
}
