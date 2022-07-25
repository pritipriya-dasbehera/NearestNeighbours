// use std::intrinsics::powf64;

// use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::time::{self, Duration};
use rand::Rng;
// use arr_macro::arr;
use ndarray::{Array3};

const NUM_OF_PARTICLES: usize = 100000;
const NEAREST_NEIGHBOURS_REQ: usize = 10;
const MAX_DIST: f64 = 0.8660254037844387;
const GRID_SIZE: usize = 46;
const GRID_LEN: f64 = 1.0/46.0;
// const MAX_DIST: f64 = 1.0;

fn main() {
    let mut now = time::Instant::now();
    let _z = now;
    let zeropt =  Point{x:0.0,y:0.0,z:0.0};
    let mut points  = vec![zeropt;NUM_OF_PARTICLES];
    point_generator(&mut points);
    // let t_point_create = now.elapsed();
    // println!("The time taken to create {} particles is {} microseconds.",NUM_OF_PARTICLES,t_point_create.as_micros());

    // now = time::Instant::now();
    write_points(&points);
    // let t_point_write = now.elapsed();
    // println!("The time taken to write {} particles is {} microseconds.",NUM_OF_PARTICLES, t_point_write.as_micros());

    // now = time::Instant::now();
    let mut nearlist = vec![Node{id:0,dist:MAX_DIST};(NEAREST_NEIGHBOURS_REQ+1)*NUM_OF_PARTICLES];
    // let t_init = now.elapsed();
    // println!("The time taken to initialise {} particle nearlist is {} microseconds.",NUM_OF_PARTICLES,t_init.as_micros());
    // print_nearlist(&nearlist);

    now = time::Instant::now();
    let mut grid = Array3::from_elem((GRID_SIZE,GRID_SIZE,GRID_SIZE), vec![0_usize]);
    create_grid(&points, &mut grid);
    let t_grid = now.elapsed();
    // println!("The time taken to make grid of {} particles is {} microseconds.",NUM_OF_PARTICLES,t_grid.as_micros());

    write_grid(&grid);

    now = time::Instant::now();
    grid_calc_nearest(&points, &grid, &mut nearlist);
    let t_gridcalc = now.elapsed();
    // println!("The time taken to grid calc {} particle nearlist is {} microseconds.",NUM_OF_PARTICLES,t_gridcalc.as_micros());

    // now = time::Instant::now();
    // brute_cal_nearest(&points, &mut nearlist);
    // let t_brute = now.elapsed();
    // println!("The time taken to brute calc {} particle nearlist is {} microseconds.",NUM_OF_PARTICLES,t_brute.as_micros());

    // now = time::Instant::now();
    write_nearlist(&nearlist);
    // let elapsed_time = now.elapsed();
    // println!("The time taken to write {} particle nearlist is {} microseconds.",NUM_OF_PARTICLES,elapsed_time.as_micros());

    // write_time(t_point_create, t_point_write, t_init, t_grid+t_gridcalc);
    write_gridtime(t_grid,t_gridcalc);
}

// #[derive(Debug)]
#[derive(Clone, Copy)]
struct Point{x: f64,y:f64,z:f64}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Node{id: u32, dist: f64}

fn dist(p1:&Point ,p2:&Point)->f64{
    let mut dx = (p1.x -p2.x).abs();
    let ax: f64 = (dx / 0.5 ).floor();
    dx = ax + (1.0 - 2.0 * ax) * dx;

    let mut dy = (p1.y -p2.y).abs();
    let ay: f64 = {dy / 0.5 }.floor();
    dy = ay + (1.0 - 2.0 * ay) * dy;


    let mut dz = (p1.z -p2.z).abs();
    let az: f64 = {dz / 0.5 }.floor();
    dz = az + (1.0 - 2.0 * az) * dz;

    // let r = {dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0)};
    // print!("The distance is {r} with dx={dx},ax={ax} dy={dy},ay={ay} and dz={dz},az={az}\n");
    // print!("The distance between p1 {:?} and p2 {:?} is {r} with dx={dx} dy={dy} and dz={dz}\n",&p1,&p2);
    dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0)
}

fn point_generator(points: &mut Vec<Point>){

    // let mut file = File::create("points.csv").expect("Something went wrong in file creation");
    // let mut file = File::options().append(true).open("points.csv").expect("Something went wrong in file creation");
    for (_i,element) in points.iter_mut().enumerate(){
        element.x = rand::thread_rng().gen::<f64>();
        element.y = rand::thread_rng().gen::<f64>();
        element.z = rand::thread_rng().gen::<f64>();
        // print!("The point {i} is {:?}\n",element);
        // let mut data = String::new();
        // data.push_str(&i.to_string());
        // data.push_str(",");
        // data.push_str(&element.x.to_string());
        // data.push_str(",");
        // data.push_str(&element.y.to_string());
        // data.push_str(",");
        // data.push_str(&element.z.to_string());
        // data.push_str("\n");
        // file.write_all(data.as_bytes()).expect("unable to write");
    }

}

// fn print_nearlist(nearlist:&Vec<Node>){
//     for i in 0..NUM_OF_PARTICLES //*(NEAREST_NEIGHBOURS_REQ+1)
//     {
//         let mut data = String::new();
//         for (j,element) in nearlist[i*(NEAREST_NEIGHBOURS_REQ+1)..i*(NEAREST_NEIGHBOURS_REQ+1)+NEAREST_NEIGHBOURS_REQ].iter().enumerate(){
//             if j==0{data.push_str(&i.to_string());continue};
//             if j==NEAREST_NEIGHBOURS_REQ+1{break;};
//             data.push(',');
//             data.push_str(&element.id.to_string());
//         }
//         data.push('\n');
//         print!("{data}");
//     }
//     print!("\n");
// }

fn write_points(points: &Vec<Point>){
    let mut file = File::create("points.csv").expect("Something went wrong in file creation");
    // let mut file = File::options().append(true).open("points.csv").expect("Something went wrong in file creation");
    file.write_all(b"id,x,y,z\n").expect("unable to write");
    for (i,element) in points.iter().enumerate(){
        let mut data = String::new();
        data.push_str(&i.to_string());
        data.push(',');
        data.push_str(&element.x.to_string());
        data.push(',');
        data.push_str(&element.y.to_string());
        data.push(',');
        data.push_str(&element.z.to_string());
        data.push('\n');
        file.write_all(data.as_bytes()).expect("unable to write");
        
        // let row = concat!(i.to_string,b",",element.x.to_be_bytes);
        // let str = i.to_be_bytes()+",".as_bytes()+element.x.to_be_bytes();
    }
}

fn write_nearlist(nearlist:&Vec<Node>){
    let mut file = File::create("nearlist.csv").expect("Something went wrong in file creation");
    // let mut file = File::options().append(true).open("nearlist.csv").expect("Something went wrong in file creation");
    // for (i, _element) in nearlist.iter().enumerate().take(NUM_OF_PARTICLES)
    for i in 0..NUM_OF_PARTICLES //*(NEAREST_NEIGHBOURS_REQ+1)
    {
        let mut data = String::new();
        for (j,element) in nearlist[i*(NEAREST_NEIGHBOURS_REQ+1)..i*(NEAREST_NEIGHBOURS_REQ+1)+NEAREST_NEIGHBOURS_REQ].iter().enumerate(){
            if j==0{data.push_str(&i.to_string());continue};
            if j==NEAREST_NEIGHBOURS_REQ+1{break;};
            data.push(',');
            data.push_str(&element.id.to_string());
        }
        data.push('\n');
        file.write_all(data.as_bytes()).expect("unable to write");
    }
}

// fn write_time(t_point_create:Duration,t_point_write:Duration,t_init:Duration,t_brute:Duration){
//     let mut file = File::options().append(true).open("time.csv").expect("Something went wrong in file creation");
//     let mut data = String::new();
//     data.push_str(&NUM_OF_PARTICLES.to_string());
//     data.push(',');
//     data.push_str(&t_point_create.as_micros().to_string());
//     data.push(',');
//     data.push_str(&t_point_write.as_micros().to_string());
//     data.push(',');
//     data.push_str(&t_init.as_micros().to_string());
//     data.push(',');
//     data.push_str(&t_brute.as_micros().to_string());
//     data.push('\n');
//     file.write_all(data.as_bytes()).expect("unable to write");
// }

fn write_gridtime(t_grid:Duration,t_gridcalc:Duration){
    let mut file = File::options().append(true).open("gridtime2.csv").expect("Something went wrong in file creation");
    let mut data = String::new();
    data.push_str(&NUM_OF_PARTICLES.to_string());
    data.push(',');
    data.push_str(&GRID_SIZE.to_string());
    data.push(',');
    data.push_str(&t_grid.as_micros().to_string());
    data.push(',');
    data.push_str(&t_gridcalc.as_micros().to_string());
    data.push('\n');
    file.write_all(data.as_bytes()).expect("unable to write");
}

fn create_grid(points: &[Point], grid:&mut Array3<Vec<usize>>){
    for (i,point) in points.iter().enumerate(){
        let x = (point.x/GRID_LEN) as usize;
        let y = (point.y/GRID_LEN) as usize;
        let z = (point.z/GRID_LEN) as usize;
        
        grid[[x,y,z]][0] += 1;
        let index = grid[[x,y,z]][0];
        grid[[x,y,z]].insert(index, i);
    }
}

fn write_grid(grid:&Array3<Vec<usize>>){
    let mut file = File::create("grid.csv").expect("Something went wrong in file creation");
    for cell in grid.iter(){
        let mut data = String::new();
        for id in cell.iter(){
            data.push_str(&id.to_string());
            data.push(',')
        }
        data.push('\n');
        file.write_all(data.as_bytes()).expect("Couldnt write the points");
    }
}

// fn write_grid(grid:&Array3<Vec<usize>>){
//     let mut file = File::create("nearlist.csv").expect("Something went wrong in file creation");
//     {0..GRID_SIZE}.for_each(|i|
//         {0..GRID_SIZE}.for_each(|j|
//             {0..GRID_SIZE}.for_each(|k|{
//                 file.write_all();
//             }
//             )))
// }

// fn brute_cal_nearest(points: &Vec<Point>, nearlist:&mut Vec<Node>){
//     // let mut file = File::create("nearlist.csv").expect("Something went wrong in file creation");
//     // let mut file = File::options().append(true).open("nearlist.csv").expect("Something went wrong in file creation");
//     // {0..NUM_OF_PARTICLES}.for_each(|i|
//     for i in 0..NUM_OF_PARTICLES
//     {
//         for j in 0..NUM_OF_PARTICLES
//         {
//             if i == j
//             {
//                 continue;
//             }
//             let d = dist(&points[i],&points[j]);
//             // print!("The distance between point {i} and point {j} is {d}\n");
//             if d<nearlist[i*(NEAREST_NEIGHBOURS_REQ+1)].dist{
//                 // print!("Inserting node for {i}\t");
//                 let mut k = 1;
//                 while k<NEAREST_NEIGHBOURS_REQ && nearlist[i*(NEAREST_NEIGHBOURS_REQ+1)+k].dist < d{
//                     k+=1;
//                 }
//                 nearlist[i*(NEAREST_NEIGHBOURS_REQ+1)+k..=i*(NEAREST_NEIGHBOURS_REQ+1)+NEAREST_NEIGHBOURS_REQ].rotate_right(1);
//                 nearlist[i*(NEAREST_NEIGHBOURS_REQ+1)+k] = Node{id:j as u32,dist:d};
//                 nearlist[i*(NEAREST_NEIGHBOURS_REQ+1)].dist = nearlist[i*(NEAREST_NEIGHBOURS_REQ+1)+NEAREST_NEIGHBOURS_REQ].dist;
//                 // print_nearlist(nearlist);
//                 // println!("\n");
//             }
//         }
//         // let mut data = String::new();
//         // for (j,element) in nearlist[i].iter().enumerate(){
//         //     if j==0{data.push_str(&i.to_string());continue};
//         //     if j==NEAREST_NEIGHBOURS_REQ+1{break;};
//         //     data.push_str(",");
//         //     data.push_str(&element.id.to_string());
//         // }
//         // data.push_str("\n");
//         // file.write_all(data.as_bytes()).expect("unable to write");
//     }
// }

fn grid_calc_nearest(points: &Vec<Point>,grid:&Array3<Vec<usize>>, nearlist:&mut Vec<Node>, ){
    let mut buffer: i64 = 1;
    (0..GRID_SIZE).for_each(|i|{
        (0..GRID_SIZE).for_each(|j|{
            (0..GRID_SIZE).for_each(|k|{
                let mut size:usize = grid[[i,j,k]].len();
                loop{
                    if size>=NEAREST_NEIGHBOURS_REQ{break;}
                    size = 0;
                    (i as i64-buffer..=i as i64+buffer).for_each(|mut ti|
                        (j as i64-buffer..=j as i64+buffer).for_each(|mut tj|
                            (k as i64-buffer..=k as i64+buffer).for_each(|mut tk|{
                                if ti < 0 {ti = GRID_SIZE as i64+ti} else if ti >= GRID_SIZE as i64 {ti = ti - GRID_SIZE as i64};
                                if tj < 0 {tj = GRID_SIZE as i64+tj} else if tj >= GRID_SIZE as i64 {tj = tj - GRID_SIZE as i64};
                                if tk < 0 {tk = GRID_SIZE as i64+tk} else if tk >= GRID_SIZE as i64 {tk = tk - GRID_SIZE as i64};
                                let ti = ti as usize;
                                let tj = tj as usize;
                                let tk = tk as usize;
                                // println!("The size is {size}");
                                size += grid[[ti,tj,tk]][1..].len();
                                // println!("The size is {size}");
                            })
                        )
                    );
                    // println!("The size is finally {size}\n\n");
                    if size>=NEAREST_NEIGHBOURS_REQ{break;}
                    // println!("Inside loop");
                    buffer+=1;
                };
                // println!("Grid {i},{j},{k} has buffer {buffer}");
                grid[[i,j,k]][1..].iter().for_each(|point1_id|{
                    let point1_id=point1_id.clone();
                    // println!("Comparing from point{point1_id}");
                    (i as i64-buffer..=i as i64+buffer).for_each(|mut ti|
                        (j as i64-buffer..=j as i64+buffer).for_each(|mut tj|
                            (k as i64-buffer..=k as i64+buffer).for_each(|mut tk|{
                                if ti < 0 {ti = GRID_SIZE as i64+ti} else if ti >= GRID_SIZE as i64 {ti = ti - GRID_SIZE as i64};
                                if tj < 0 {tj = GRID_SIZE as i64+tj} else if tj >= GRID_SIZE as i64 {tj = tj - GRID_SIZE as i64};
                                if tk < 0 {tk = GRID_SIZE as i64+tk} else if tk >= GRID_SIZE as i64 {tk = tk - GRID_SIZE as i64};
                                let ti = ti as usize;
                                let tj = tj as usize;
                                let tk = tk as usize;
                                for point2_id in grid[[ti,tj,tk]][1..].iter(){
                                    let point2_id=point2_id.clone();
                                    // if n==0{ if point2_id==0{break;} else{continue;};};
                                    if point1_id==point2_id{continue;}
                                    // println!("In grid {i},{j},{k} point{point1_id} compared to point{point2_id} in grid {ti},{tj},{tk} with buffer{buffer}");
                                    let d = dist(&points[point1_id],&points[point2_id]);
                                    if d<nearlist[point1_id*(NEAREST_NEIGHBOURS_REQ+1)].dist{
                                        // print!("Inserting node for {i}\t");
                                        let mut k = 1;
                                        while k<NEAREST_NEIGHBOURS_REQ && nearlist[point1_id*(NEAREST_NEIGHBOURS_REQ+1)+k].dist < d {k+=1;}
                                        nearlist[point1_id*(NEAREST_NEIGHBOURS_REQ+1)+k..=point1_id*(NEAREST_NEIGHBOURS_REQ+1)+NEAREST_NEIGHBOURS_REQ].rotate_right(1);
                                        nearlist[point1_id*(NEAREST_NEIGHBOURS_REQ+1)+k] = Node{id:point2_id as u32,dist:d};
                                        nearlist[point1_id*(NEAREST_NEIGHBOURS_REQ+1)].dist = nearlist[point1_id*(NEAREST_NEIGHBOURS_REQ+1)+NEAREST_NEIGHBOURS_REQ].dist;
                                    }
                                }
                            })
                        )
                    );
                });
            })
        })
    });
}