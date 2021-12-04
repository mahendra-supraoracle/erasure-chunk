use std::io;
use rand::Rng;
use reed_solomon_erasure::galois_8::ReedSolomon;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::str;

extern crate time;

#[macro_use(shards)]
extern crate reed_solomon_erasure;

// TEST CODE
// or use the following for Galois 2^16 backend
// use reed_solomon_erasure::galois_16::ReedSolomon;

// fn main() {
    
//     println!("Hello, world!");

//     // let secret_number = rand::thread_rng().gen_range(1..101);

//     // println!("your secret number : {}", secret_number);

//     // println!("Enter some input..");

//     // let mut guess = String::new();

//     // io::stdin()
//     // .read_line(&mut guess)
//     // .expect("error while reading input");

//     // //
//     // //
//     // println!("You guess : {}", guess);

//     //
//     let r = ReedSolomon::new(13, 5).unwrap(); // 3 data shards, 2 parity shards

//     // let mut master_copy = shards!(
//     //     [0, 1,  2,  3],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [4, 5,  6,  7],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [8, 9, 10, 11],
//     //     [0, 0,  0,  0], // last 2 rows are parity shards
//     //     [0, 0,  0,  0],
//     //     [0, 0,  0,  0],
//     //     [0, 0,  0,  0],
//     //     [0, 0,  0,  0]
//     // );

//     let mut master_copy = shards!(
//         {
//             "name":"mahendra"
//         }
//     );

//     // Construct the parity shards
//     r.encode(&mut master_copy).unwrap();

//     // Make a copy and transform it into option shards arrangement
//     // for feeding into reconstruct_shards
//     let mut shards: Vec<_> = master_copy.iter().cloned().map(Some).collect();

//     // We can remove up to 2 shards, which may be data or parity shards
//     //shards[0] = None;
//     //shards[1] = None;
//     shards[2] = None;
//      shards[3] = None;
//     // shards[4] = None;
//     // shards[5] = None;
//     // shards[6] = None;
//     // shards[7] = None;
//     // shards[8] = None;
//      shards[9] = None;
//      shards[10] = None;
//      shards[11] = None;
//     // shards[12] = None;
//     // shards[13] = None;
//     // shards[14] = None;
//     // shards[15] = None;
//     //shards[16] = None;
//     //shards[17] = None;
    
//     println!("shards lenght : {}", shards.len());

//     // Try to reconstruct missing shards
//     r.reconstruct(&mut shards).unwrap();

//     // Convert back to normal shard arrangement
//     let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

//     assert!(r.verify(&result).unwrap());
//     assert_eq!(master_copy, result);

// }


macro_rules! make_random_parity_shards {
    ($per_shard:expr, $size:expr) => {{
        let mut shards = Vec::<Vec<u8>>::with_capacity(13);
        for _ in 0..$size {
            shards.push(vec![0u8; $per_shard]);
        }

        for s in shards.iter_mut() {
            fill_random(s);
        }
        shards
    }}
}

fn fill_random(arr : &mut [u8]) {
    for a in arr.iter_mut() {
        *a = 0;
    }
}

fn main() -> std::io::Result<()> {

    // TRANSACTION SOURCE FILE
    let file = File::open("transactions.json")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    //println!("FILE CONTENT ::: {} ", contents);

    let start = time::precise_time_ns();

    let src: Vec<u8> = contents.to_owned().as_bytes().to_vec();
    println!("Batch Size ( Bytes ) :: {:?}", src.len());

    let dst: Vec<Vec<u8>> = src.chunks(850210).map(|x| x.to_vec()).collect();       // CURRENT SIZE OF DATA 01_39_000
    
    // CHUNK ARRAY
    // VERIFYING SIZE
    // println!("DST LEN :: {:?}", dst.len() );
    // println!("DST INTERNAL DATA LEN  1 :: {:?}", dst[0].len() );
    // println!("DST INTERNAL DATA LEN  2 :: {:?}", dst[1].len() );
    // println!("dst : {:?}", dst.len());

    // REFERENCED
    let mut master_copy = dst;

    let r = ReedSolomon::new(8, 5).unwrap();                                        // 8 data shards, 5 parity shards

    // DYNAMIC PARITY PARAMS 
    let mut dyn_shards = make_random_parity_shards!(850210, 5);                     // GENERATE 5 PARITY SHARD WITH 0's
    //
    for shard_with_zero_element in dyn_shards.iter_mut() {
        master_copy.push(shard_with_zero_element.to_vec());
    }


    println!("master_copy: total chunk generated {:?}", master_copy.len());
    
    //
    println!("master_copy: 0 {:?}", master_copy[0].len());
    println!("master_copy: 1 {:?}", master_copy[1].len());
    println!("master_copy: 2 {:?}", master_copy[2].len());
    println!("master_copy: 3 {:?}", master_copy[3].len());
    println!("master_copy: 4 {:?}", master_copy[4].len());
    println!("master_copy: 5 {:?}", master_copy[5].len());
    println!("master_copy: 6 {:?}", master_copy[6].len());
    println!("master_copy: 7 {:?}", master_copy[7].len());
    println!("master_copy: 8 {:?}", master_copy[8].len());
    println!("master_copy: 9 {:?}", master_copy[9].len());
    println!("master_copy: 10 {:?}", master_copy[10].len());
    println!("master_copy: 11 {:?}", master_copy[11].len());
    println!("master_copy: 12 {:?}", master_copy[12].len());
    
    // Construct the parity shards
    r.encode(&mut master_copy).unwrap();

    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = master_copy.iter().cloned().map(Some).collect();

    // We can remove up to few shards, which may be data or parity shards
    shards[1] = None;
    shards[3] = None;
    shards[5] = None;
    shards[7] = None;

    // let mut abc = vec![];
    // abc.push(shards[0].clone());
    // abc.push(shards[1].clone());
    // abc.push(shards[2].clone());
    // abc.push(shards[3].clone());
    // abc.push(shards[4].clone());
    // abc.push(shards[5].clone());
    // abc.push(shards[6].clone());
    // abc.push(shards[7].clone());
    // abc.push(shards[8].clone());
    // abc.push(shards[9].clone());
    // abc.push(shards[10].clone());
    // abc.push(shards[11].clone());
    // abc.push(shards[12].clone());

    // println!("shards: {:?}", abc);
    // Try to reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();
    //println!("result: {:?}", result);
    println!("result: {:?}", result.len());
    
    // MERGE 
        // Collect the results
        // let mut resultsss = Vec::with_capacity(result.len());
        // for g in result {
        //     resultsss.extend(g.join().unwrap().into_iter());
        // }

        // println!("resultttttt : {:?}", resultsss.len());
    // END MERGE
    
    let output_src = str::from_utf8(&result[0]).unwrap();    // BYTES TO ORIGINAL DATA

    //println!("O/T : {}", output_src);

    //
    let end   = time::precise_time_ns();
    let time_taken = (end - start) as f64 / 1_000_000_000.0;
    
    //
    println!("    time taken       : {}", time_taken);

    assert!(r.verify(&result).unwrap());
    //println!("master_copy: {:?}", master_copy);
    assert_eq!(master_copy, result);
    println!("Result (OK) ");
    Ok(())
}    