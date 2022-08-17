mod lib;

fn main() {
    // let gif = BitMapBackend::gif("test.gif", (500, 500), 500).expect("Error creating gif object");

    // let root = gif.into_drawing_area();


    // let mut values = vec![];
    //     for i in 0..20 {
    //     values.push(i)
    // }
    // let area = root.margin(40, 30, 20, 10); // Give the main plot some padding on each side.
    // area.fill(&BLACK).unwrap(); // Fill the main plot area with white color.
    
    // const W: usize = 45;
    // const H: usize = 30;
    // const NC: usize = 10;
    // const NR: usize = 10;
    
    // for i in 0..values.len() { // Make a 10x10 plot of 30px by 30px squares.
    //     let v = values[i];
    //     let row = i / NC; // Current row index
    //     let col = i % NC; // Current column index
        
    //     let lt = ((col * W) as i32, (row * H) as i32); // Top left coordinate of this cell
    //     let rb = (lt.0 + W as i32, lt.1 + H as i32); // Bottom right coordinate of this cell
    //     let cc = Cell{lt, rb, border_color: RED, background_color: CYAN, 
    //         text: Some(v.to_string()), font:Some(("Courier".to_string(), 15, Color(0, 0, 0, 255)))};
    //     cc.draw(&area);
    //         root.present().unwrap();
    // }

    // println!("{:?}", crate::lib::GIFWrapper("path".to_string(), "".to_string()));
}
