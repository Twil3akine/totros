use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
struct Totros {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}

#[allow(dead_code)]
impl Totros {
    /*
     Totrosの初期化
     */
    fn new() -> Self {
        let w: usize = 11;
        let h: usize = 16;

        Totros {
            width: w,
            height: h,
            grid: vec![vec![false; w]; h],
        }
    }

    /*
     grid全体を出力
     */
    fn print(&self) {
        // 横線関数の定義
        let write_line = |w: usize| {
            print!("+");
            for _ in 0..w { print!("-"); }
            println!("+");
        };

        write_line(self.width);
        // 縦ループ
        for h in 0..self.height {
            // 横ループ
            print!("|");
            for w in 0..self.width {
                print!("{}", if self.grid[h][w] { '#' } else { ' ' });
            }
            println!("|");
        }
        write_line(self.width);
    }

    fn update(&self) -> bool {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");

        self.print();
        
        sleep(Duration::from_secs(1));

        true
    }
}

fn main() {
    let mut totros = Totros::new();

    let width: usize = totros.width;
    let height: usize = totros.height;

    while totros.update() {}
}