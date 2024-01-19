use bomboni::wasm::Wasm;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[derive(Debug)]
#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

#[wasm_bindgen]
impl Task {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            done: false,
        }
    }
}

// error: only C-Style enums allowed with #[wasm_bindgen]
// #[wasm_bindgen]
// pub enum InvalidEnum {
//     Message(String),
//     Status(u32),
// }

#[derive(Debug, Clone, Serialize, Deserialize, Wasm)]
#[serde(tag = "type", content = "value", rename_all = "SCREAMING_SNAKE_CASE")]
#[wasm(wasm_abi)]
pub enum Value {
    String(String),
    Number(f64),
    Reference { column: u32, row: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, Wasm)]
#[wasm(wasm_abi)]
pub struct Cell {
    pub id: (u32, u32),
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize, Wasm)]
#[wasm(wasm_abi)]
pub struct Grid(Vec<Vec<Cell>>);

#[wasm_bindgen]
pub struct Sheet {
    grid: Grid,
}

#[wasm_bindgen]
impl Sheet {
    #[wasm_bindgen(constructor)]
    pub fn new(grid: Grid) -> Self {
        for row in grid.0.iter() {
            assert_eq!(row.len(), grid.0[0].len());
        }
        Self { grid }
    }

    pub fn get_average(&self, column: u32) -> f64 {
        let mut avg = 0.0;
        for row in &self.grid.0 {
            if let Some(cell) = row.get(column as usize) {
                if let Value::Number(n) = &cell.value {
                    avg += *n / row.len() as f64;
                }
            }
        }
        avg
    }

    pub fn get_column(&self, column: u32) -> Vec<Cell> {
        self.grid
            .0
            .iter()
            .map(|row| row[column as usize].clone())
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        if self.grid.0.is_empty() {
            return 0;
        }
        self.grid.0[0].len() as u32
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.grid.0.len() as u32
    }
}

#[derive(Debug, Serialize, Deserialize, Wasm)]
#[wasm(as_string)]
pub struct ItemId(u128);

impl Display for ItemId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl FromStr for ItemId {
    type Err = <u128 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u128::from_str_radix(s, 16).map(Self)
    }
}

#[wasm_bindgen]
pub fn make_id() -> ItemId {
    ItemId(rand::random())
}
