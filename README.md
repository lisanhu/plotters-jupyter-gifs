# plotters-jupyter-gifs
GIF extension for plotters-rs on jupyter notebooks that enables to display gifs in jupyter notebooks

# Sample demo
```rust
:dep plotters = { git = "https://github.com/plotters-rs/plotters.git", default_features = false, features = ["evcxr_bitmap", "all_series"] }
:dep plotters-jupyter-gifs = { path = "/home/lisanhu/mine/tmp/plotters-jupyter-gifs" }
use plotters::prelude::*;
use plotters::evcxr::*;
use plotters_jupyter_gifs::*;
use plotters::style::text_anchor::{Pos, HPos, VPos};
```
```rust
use plotters_jupyter_gifs::{Cell, Color, CYAN, RED, GREEN, BLUE};

let mut values = vec![];
for i in 0..100 {
    values.push(i)
}

let figure = plotters_jupyter_gifs::evcxr_gif_figure((500, 500), "tmp", 100, |root| {
    let area = root.margin(40, 30, 20, 10); // Give the main plot some padding on each side.
//     area.fill(&BLACK)?; // Fill the main plot area with white color.
    
    const W: usize = 45;
    const H: usize = 30;
    const NC: usize = 10;
    const NR: usize = 15;
    
    for i in 0..values.len() { // Make a 10x10 plot of 30px by 30px squares.
        let v = values[i];
        let row = i / NC; // Current row index
        let col = i % NC; // Current column index
        
        let lt = ((col * W) as i32, (row * H) as i32); // Top left coordinate of this cell
        let rb = (lt.0 + W as i32, lt.1 + H as i32); // Bottom right coordinate of this cell
        let cc = Cell{lt, rb, border_color: RED, background_color: CYAN, 
            text: Some(v.to_string()), font:Some(("Courier".to_string(), 15, Color(0, 0, 0, 255)))};
        cc.draw(&area);
        root.present()?;
    }
    
    Ok(())
    });
figure
```
`evcxr_gif_figure` parameters: dimension of the gif, generated gif path, time between frames in milliseconds, a lambda expression to draw on the context (see plotters documentation for details)
