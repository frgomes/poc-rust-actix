/*
 * actix-web demo
 *
 * actix-web server and client demo
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: nospam@example.com
 * Generated by: https://openapi-generator.tech
 */

/// DataTable : Data table.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataTable {
    /// Table name.
    #[serde(rename = "name")]
    pub name: String,
    /// List of cells.
    #[serde(rename = "cells")]
    pub cells: Vec<crate::models::DataCell>,
}

impl DataTable {
    /// Data table.
    pub fn new(name: String, cells: Vec<crate::models::DataCell>) -> DataTable {
        DataTable {
            name,
            cells,
        }
    }
}


