// use std::boxed::Box;
// use std::vec::Vec;
// use std::ops::Drop

mod solver;
use solver::solve_quadratic_by_imaginary;

// struct List {
//     mbox: Box<[u16]>
// }
// trait Overloadable<T> {
//     fn to_malloc_box(&self, _: T);
// }
// impl Overloadable<usize> for List {
//     fn to_malloc_box(&self, n: usize) {
//         let mbox = vec![0; n].into_boxed_slice();
//         println!("{:?}", self.mbox);
//         println!("{:?}", mbox);
//         &self.mbox = &mbox;
//         // return self.mbox;
//     }
// }
// impl Overloadable<Vec<u16>> for List {
//     fn to_malloc_box(&self, vec: Vec<u16>) {
//         let mbox = vec.into_boxed_slice();
//         println!("{:?}", self.mbox);
//         println!("{:?}", mbox);
//         &self.mbox = &mbox;
//         // return self.mbox;
//     }  
// }



// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Drop for Point {
//     fn drop(&mut self) { // Pointが解放される際に呼び出される
//         println!("Bye!");
//     }
// }



// pub fn get_array_simple_loop(_start: u16, _end: u16) -> Box<[u16]> {
//     let i_start: u16 = _start - _start;
//     let i_end: u16 = _end - _start;
//     let i_end_usize: usize = i_end as usize;
//     // let mary: &mut [u16];
//     // let mut mary: [u16; _end - _start];
//     let mut mvec: Vec<u16> = Vec::with_capacity(i_end_usize);
//     for i in i_start..i_end {
//         mvec.push(i + i_start);
//     }
//     let lst: List;
//     // let mbox = lst.to_malloc_box(mvec);
//     lst.to_malloc_box(mvec);
//     // return mary;
//     return lst.mbox;
// }

fn solve_equation() -> bool{
    const A: i64 = 1;
    const B: i64 = 3;
    const C: i64 = 10;
    let sol_tup_co = solve_quadratic_by_imaginary(A, B, C);
    // let sol_tup_re = solver::solve_quadratic_by_real(A, B, c);
    // println!("Readl solutions, {} !", sol_tup_re.0);
    // println!("Readl solutions, {} !", sol_tup_re.1);
    println!("Imaginary solutions, {} !", sol_tup_co.0);
    println!("Imaginary solutions, {} !", sol_tup_co.1);
    return true;
}

fn main() {
    let _ = solve_equation();
    // https://doc.rust-lang.org/std/keyword.struct.html
    
    const A: u16 = 1_000;
    const B: u16 = 5_000;
    const C: u16 = 20_000;
    println!("Hello, world!");
    println!("Hello, {} !", solve_quadratic(A, B, C));
    // let mbox = get_array_simple_loop(0, 10);
    // for i in 0..10 {
    //     println!("{}", i);
    // }
    // println!("Array {:?}", mbox);
}
