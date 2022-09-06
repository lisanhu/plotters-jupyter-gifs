mod lib;
use lib::*;



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
