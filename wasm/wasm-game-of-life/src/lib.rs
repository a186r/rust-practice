mod utils;
use std::fmt;
use wasm_bindgen::__rt::core::fmt::{Error, Formatter};
use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//将console.log封装成一个println!风格的宏
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
//每个单元格被表示为单个字节
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

/// 定义宇宙
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

pub struct Timer<'a> {
    name: &'a str,
}

/// 公共函数，暴露给javascript
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        //每次调用Universe::tick花费的时间输出到控制台
        let _timer = Timer::new("Universe::tick");

        let mut next = {
            let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        //    规则1，任何存活细胞周围少于2个存活邻居则死亡，仿佛是人口过少造成的
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        //    规则2，任何存活细胞周围有2个或者3个存活邻居则活到下一代
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        //规则3，任何存活细胞周围有超过3个存活邻居则死亡，仿佛是人口过剩造成的
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        //    规则4，任何死亡细胞周围有正好3个存活细胞则变成存活细胞，仿佛是生殖
                        (Cell::Dead, 3) => Cell::Alive,
                        //    其他所有单元格保持同样的状态
                        (otherwise, _) => otherwise,
                    };
                    log!("    it becomes {:?}", next_cell);
                    next[idx] = next_cell;
                }
            }
        }
        let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    //为了访问给定行列的单元格，将行列翻译为单元格向量的索引
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    //为了计算单元格的下一个状态，需要得到一个有多少邻居存活的计数
    //消耗过多资源，重写此方法
    // fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    //     let mut count = 0;
    //     for delta_row in [self.height - 1, 0, 1].iter().cloned() {
    //         for delta_col in [self.width - 1, 0, 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }
    //
    //             let neighbor_row = (row + delta_row) % self.height;
    //             let neighbor_col = (column + delta_col) % self.width;
    //             let idx = self.get_index(neighbor_row, neighbor_col);
    //             count += self.cells[idx] as u8;
    //         }
    //     }
    //     count
    // }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    //构造函数
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// 设置宇宙的宽度
    ///
    /// 重置所有细胞为Dead状态
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// 设置宇宙的高度
    ///
    /// 重置所有细胞为Dead状态
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..height * self.height).map(|_i| Cell::Dead).collect();
    }

    /// 使细胞变异
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

//不使用#[wasm_bindgen]属性来创建另一个impl Universe块，有一些我们想测试需要但不想暴露给JS的函数。
impl Universe {
    /// 获取整个宇宙中生存和死亡数
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// 通过将行和列以数组的形式传递给宇宙中每一个需要设为存活的细胞
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

//切换细胞状态
impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name)
    }
}
