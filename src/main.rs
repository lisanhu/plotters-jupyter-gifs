mod lib;
use lib::*;

struct SWChart {
    //  containing size info, so string with whitespaces when not wishing to show col or row text
    pub col_text: String,
    pub row_text: String,
    pub label: String,
    pub values: Vec<i32>,
}

impl SWChart {
    pub fn new(col: &str, row: &str, label: &str, vals: Vec<i32>) -> SWChart {
        SWChart {
            col_text: col.into(),
            row_text: row.into(),
            label: label.into(),
            values: vals,
        }
    }

    pub fn draw(&self, area: &DrawingArea<BitMapBackend, Shift>) {
        let root = area.margin(10, 10, 10, 10);

        const W: usize = 48;
        const H: usize = 30;
        let font = ("Courier".to_owned(), 15, BLACK);
        let num_rows = self.col_text.len() + 1;
        let num_cols = self.row_text.len() + 1;
        assert!(
            num_rows * num_cols <= self.values.len(),
            "Not enough values to fill the tiles! {}/{}",
            self.values.len(),
            num_rows * num_cols
        );

        let lt = (0, 0);
        let rb = (lt.0 + W as i32, lt.1 + H as i32);
        let mut cc = Cell {
            lt,
            rb,
            border_color: RED,
            background_color: WHITE,
            text: None,
            font: None,
        };
        cc.draw(&root);

        let lt = (W as i32, 0);
        let rb = (lt.0 + W as i32, lt.1 + H as i32);
        cc.lt = lt;
        cc.rb = rb;
        cc.draw(&root);

        let lt = (0, H as i32);
        let rb = (lt.0 + W as i32, lt.1 + H as i32);
        cc.lt = lt;
        cc.rb = rb;
        cc.draw(&root);

        for (i, c) in self.col_text.chars().enumerate() {
            let i = i + 2;
            let lt = ((i * W) as i32, 0);
            let rb = (lt.0 + W as i32, lt.1 + H as i32);
            let cc = Cell {
                lt,
                rb,
                border_color: RED,
                background_color: WHITE,
                text: Some(c.to_string()),
                font: Some(font.clone()),
            };
            cc.draw(&root);
        }

        for (i, c) in self.row_text.chars().enumerate() {
            let i = i + 2;
            let lt = (0, (i * H) as i32);
            let rb = (lt.0 + W as i32, lt.1 + H as i32);
            let cc = Cell {
                lt,
                rb,
                border_color: RED,
                background_color: WHITE,
                text: Some(c.to_string()),
                font: Some(font.clone()),
            };
            cc.draw(&root);
        }

        for i in 0..num_rows + num_cols - 1 {
            for j in 0..i + 1 {
                if i >= j && j < num_cols && i - j < num_rows {
                    let x = j;
                    let y = i - j;
                    let lt = ((x * W + W) as i32, (y * H + H) as i32); // Top left coordinate of this cell
                    let rb = (lt.0 + W as i32, lt.1 + H as i32); // Bottom right coordinate of this cell

                    let cc = Cell {
                        lt,
                        rb,
                        border_color: RED,
                        background_color: CYAN,
                        text: Some(self.values[y * num_cols + x].to_string()),
                        font: Some(font.clone()),
                    };
                    cc.draw(&root);
                }
            }
        }

        let lt = (0, ((num_rows + 1) * H) as i32);
        let rb = ((lt.0 + ((num_cols + 1) * W) as i32), lt.1 + H as i32);
        let cc = Cell {
            lt,
            rb,
            border_color: RED,
            background_color: WHITE,
            text: Some(self.label.clone()),
            font: Some(font.clone()),
        };
        cc.draw(&root);

        // root.present().unwrap();
    }
}

fn main() {
    let cfg = Config {
        go: 4,
        ge: 1,
        ma: 2,
        mi: 1,
    };
    let mat = create_matrix_swg(Query, Target, cfg);

    // width, height
    let figure = evcxr_gif_figure((900, 1200), "tmp-a", 1000, |root| {
        // let figure = evcxr_bitmap_figure((676, 424), |root| {
        let area = root.margin(2, 2, 2, 2); // Give the main plot some padding on each side.
        let areas = area.split_evenly((4, 2));
        // let num_rows = mat.len();
        // let num_cols = mat[0].len();

        const W: usize = 48;
        const H: usize = 30;

        // for i in 0..num_rows + num_cols - 1 {
        //     for j in 0..i + 1 {
        //         if i >= j && j < num_cols && i - j < num_rows {}
        //     }
        //         println!();
        let extracted: Vec<CellData> = mat.iter().flat_map(|celldata| celldata.clone()).collect();
        let extracted: Vec<i32> = extracted.iter().map(|celldata| celldata.h).collect();
        let chart = SWChart::new(Query, Target, "H", extracted);
        chart.draw(&areas[0]);
        let extracted: Vec<CellData> = mat.iter().flat_map(|celldata| celldata.clone()).collect();
        let extracted: Vec<i32> = extracted.iter().map(|celldata| celldata.e).collect();
        let chart = SWChart::new(Query, Target, "E", extracted);
        chart.draw(&areas[1]);
        let extracted: Vec<CellData> = mat.iter().flat_map(|celldata| celldata.clone()).collect();
        let extracted: Vec<i32> = extracted.iter().map(|celldata| celldata.f).collect();
        let chart = SWChart::new(Query, Target, "F", extracted);
        chart.draw(&areas[2]);
        root.present()?;
        // }

        Ok(())
    });
    figure.evcxr_display();
    // return;
}

#[derive(Copy, Clone, Debug)]
pub struct CellData {
    e: i32,
    f: i32,
    h: i32,
    i: usize,
    j: usize,
}

pub struct Config {
    pub go: i32,
    pub ge: i32,
    pub ma: i32,
    pub mi: i32,
}

const Query: &str = "GAATTC";
const Target: &str = "GAATTC";
const INF: i32 = i32::MAX / 2;

use std::{cell, cmp::max};

use plotters::{
    coord::Shift,
    prelude::{BitMapBackend, DrawingArea},
};

use crate::lib::{BLACK, RED};

pub fn create_matrix_swg(query: &str, target: &str, cfg: Config) -> Vec<Vec<CellData>> {
    let mut res = Vec::new();

    for i in 0..target.len() + 1 {
        let mut row = Vec::new();
        for j in 0..query.len() + 1 {
            let mut e = 0;
            let mut f = 0;
            let mut h = 0;

            if i == 0 && j != 0 {
                h = -cfg.go - j as i32 * cfg.ge;
                e = -INF;
                f = -cfg.go - j as i32 * cfg.ge;
            } else if i != 0 && j == 0 {
                h = -cfg.go - i as i32 * cfg.ge;
                e = -cfg.go - i as i32 * cfg.ge;
                f = -INF;
            }

            row.push(CellData { e, f, h, i, j });
        }
        res.push(row);
    }

    for i in 1..res.len() {
        for j in 1..res[i].len() {
            let mut ms = if query.as_bytes()[j - 1] == target.as_bytes()[i - 1] {
                cfg.ma
            } else {
                -cfg.mi
            };
            let h = max(
                max(res[i - 1][j].e, res[i][j - 1].f),
                res[i - 1][j - 1].h + ms,
            );
            let e = max(h - cfg.go, res[i - 1][j].e - cfg.ge);
            let f = max(h - cfg.go, res[i][j - 1].f - cfg.ge);

            res[i][j] = CellData { e, f, h, i, j };
        }
    }

    return res;
}
