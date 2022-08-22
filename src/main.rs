mod lib;

use plotters::evcxr::evcxr_bitmap_figure;

fn main() {
    let cfg = Config {
        go: 4,
        ge: 1,
        ma: 2,
        mi: 1,
    };
    let mat = create_matrix_swg(Query, Target, cfg);

    use plotters_jupyter_gifs::evcxr_gif_figure;
    use plotters_jupyter_gifs::{Cell, Color, BLUE, CYAN, GREEN, RED};
    // let figure = evcxr_gif_figure((676, 424), "tmp", 1000, |root| {
    let figure = evcxr_bitmap_figure((676, 424), |root| {
        let area = root.margin(2, 2, 2, 2); // Give the main plot some padding on each side.
                                            //     area.fill(&plotters::style::RGBColor(0, 0, 0))?; // Fill the main plot area with white color.
                                            //     let areas = root.split_horizontally(250);
                                            //     use plotters::style::Color;
                                            //     area.fill(&WHITE)?;
        let areas = area.split_evenly((2, 2));
        let num_rows = mat.len();
        let num_cols = mat[0].len();

        const W: usize = 48;
        const H: usize = 30;

        for i in 0..num_rows + num_cols - 1 {
            for j in 0..i + 1 {
                if i >= j && j < num_cols && i - j < num_rows {
                    let data = mat[i - j][j];
                    let row = data.i;
                    let col = data.j;
                    let lt = ((col * W) as i32, (row * H) as i32); // Top left coordinate of this cell
                    let rb = (lt.0 + W as i32, lt.1 + H as i32); // Bottom right coordinate of this cell
                                                                 //             println!("{},{},{}:{:?}", row, col, data.h, (lt, rb));
                    let cc = Cell {
                        lt,
                        rb,
                        border_color: RED,
                        background_color: CYAN,
                        text: Some(data.h.to_string()),
                        font: Some(("Courier".to_string(), 15, Color(0, 0, 0, 255))),
                    };
                    cc.draw(&areas[0]);

                    let cc = Cell {
                        lt,
                        rb,
                        border_color: RED,
                        background_color: CYAN,
                        text: Some(data.e.to_string()),
                        font: Some(("Courier".to_string(), 15, Color(0, 0, 0, 255))),
                    };
                    cc.draw(&areas[1]);

                    let cc = Cell {
                        lt,
                        rb,
                        border_color: RED,
                        background_color: CYAN,
                        text: Some(data.f.to_string()),
                        font: Some(("Courier".to_string(), 15, Color(0, 0, 0, 255))),
                    };
                    cc.draw(&areas[2]);
                }
            }
            //         println!();
            root.present()?
        }

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

use std::cmp::max;

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
            let e = std::cmp::max(res[i][j].h - cfg.go, res[i - 1][j].e - cfg.ge);
            let f = std::cmp::max(res[i][j].h - cfg.go, res[i][j - 1].f - cfg.ge);
            let h = max(
                max(res[i - 1][j].e, res[i][j - 1].f),
                res[i - 1][j - 1].h + ms,
            );

            res[i][j] = CellData { e, f, h, i, j };
        }
    }

    return res;
}
