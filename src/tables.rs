use crate::gh::ListModuleResponse;
use prettytable::{cell, row, Cell, Row, Table};

// pub fn print_modules_table(list_module_items: Vec<ListModuleItem>) {
//     let mut table = Table::new();
//     table.add_row(row!["Name", "Description", "URL"]);
//     for module in list_module_items {
//         let mut row = Row::empty();
//         row.add_cell(Cell::new(&module.name));
//         row.add_cell(Cell::new(&module.description.unwrap_or("".to_string())));
//         row.add_cell(Cell::new(&module.html_url));
//         table.add_row(row);
//     }
//     table.printstd();
// }

fn add_header(table: &mut Table) {
    table.add_row(row!["Name", "Description", "URL"]);
}

fn add_footer(table: &mut Table, total_count: u64) {
    let total_count_cell = format!("Total Count: {}", total_count);
    table.add_row(row![total_count_cell]);
}

pub fn print_modules_table(list_modules_response: ListModuleResponse) {
    let mut table = Table::new();
    add_header(&mut table);
    for module in list_modules_response.items {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&module.name));
        row.add_cell(Cell::new(&module.description.unwrap_or("".to_string())));
        row.add_cell(Cell::new(&module.html_url));
        table.add_row(row);
    }
    add_footer(&mut table, list_modules_response.total_count);
    table.printstd();
}
