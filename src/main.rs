use std::collections::VecDeque;

fn main() {
    let font =
        bdf_parser::BdfFont::parse(include_bytes!("../fonts/Gallant19.bdf"))
            .unwrap();
    let gsz = font.glyphs.get('W').as_ref().unwrap().bounding_box.size;

    for arg in std::env::args().skip(1) {
        let mut ll = VecDeque::new();
        while ll.len() < (gsz.y as usize) {
            ll.push_back(String::new());
        }

        for c in arg.chars() {
            if let Some(g) = font.glyphs.get(c).or_else(|| font.glyphs.get(' '))
            {
                for y in 0..(g.bounding_box.size.y as usize) {
                    for x in 0..(g.bounding_box.size.x as usize) {
                        if g.pixel(x, y) {
                            ll[y].push_str("\x1b[1;37;47m#\x1b[0m");
                        } else {
                            ll[y].push(' ');
                        }
                    }
                    ll[y].push(' ');
                }
            }
        }

        while ll
            .iter()
            .next()
            .map(|s| s.chars().all(|c| c == ' '))
            .unwrap_or(false)
        {
            ll.pop_front();
        }
        while ll
            .iter()
            .last()
            .map(|s| s.chars().all(|c| c == ' '))
            .unwrap_or(false)
        {
            ll.pop_back();
        }

        for l in ll {
            println!("{}", l);
        }
        println!();
    }
}
