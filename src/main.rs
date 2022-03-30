use std::env::{args, Args};
use reed_solomon_erasure::galois_8::ReedSolomon;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Instant;

// use std::str;
// use std::str::from_utf8;

extern crate time;

//#[macro_use(shards)]
extern crate reed_solomon_erasure;

macro_rules! make_random_parity_shards {
    ($per_shard:expr, $size:expr) => {{
        let mut shards = Vec::<Vec<u8>>::with_capacity(13);
        shards = vec![vec![0u8; $per_shard]; $size];
        shards
    }}
}


//
//  [N, K]
//  N   :   erasure code encodes source data into {N} shards, where (N ≥ 1)
//  K   :   chunk size decided 1/K, where 1 is source data size{MB}, k is parity chunk (1 ≤ K ≤ N)
//
fn main() -> std::io::Result<()> {

    // ERASURE CHUNK CONFIGURATION
    let mut _args: Args = args();
    println!("Arguments : {:?}", _args);
    //println!("Hello, world...!");
    let n = _args.nth(1).unwrap().parse::<usize>().unwrap();
    let k = _args.nth(0).unwrap().parse::<usize>().unwrap();
    let file_name = _args.nth(0).unwrap().to_string(); //String.from_utf8()

    println!("ARGUMENT :- N = {:?}  K = {:?} File Name = {:?}", n, k, file_name);

//     if n <= k || k <= 0 {
//         panic!("N,K Value must be followed (1 ≤ K ≤ N)");
//     }

    // TRANSACTION SOURCE FILE
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // START TIME
    let start = time::precise_time_ns();
    let start_1 = Instant::now();

    let mut src: Vec<u8> = contents.to_owned().as_bytes().to_vec();

    let _mod:usize = src.len() % n;                                        // BATCH SIZE ADJUSTMENT
    if _mod != 0 {
        let _mod1:usize = n - _mod;
        src.extend(std::iter::repeat(0).take(_mod1));

    }

    let per_chunk_size:usize = src.len() / n;

    // CREATE CHUNK
    let mut master_copy: Vec<Vec<u8>> = src.chunks(per_chunk_size).map(|x| x.to_vec()).collect();       // CURRENT SIZE OF DATA 01_39_000
    
    // CHUNK ARRAY
    // VERIFYING SIZE
    // println!("DST LEN :: {:?}", master_copy.len() );
    // println!("DST INTERNAL DATA LEN  1 :: {:?}", master_copy[0].len() );
    // println!("DST INTERNAL DATA LEN  2 :: {:?}", master_copy[1].len() );
    // println!("dst : {:?}", master_copy.len());

    // REFERENCED
    //let mut master_copy = dst;

    let r = ReedSolomon::new(n, k).unwrap();                     // 8 data shards, 5 parity shards

    // GENERATE DYNAMIC PARITY CHUNK
    let mut parity_shards = make_random_parity_shards!(per_chunk_size, k);                     // GENERATE 5 PARITY SHARD WITH 0's

    //
    master_copy.append(&mut parity_shards);
    //
    //
    // println!("master_copy: total chunk generated {:?}", master_copy.len());
    // //
    // println!("master_copy: 0 {:?}", master_copy[0].len());
    // println!("master_copy: 1 {:?}", master_copy[1].len());
    // println!("master_copy: 2 {:?}", master_copy[2].len());
    // println!("master_copy: 3 {:?}", master_copy[3].len());
    // println!("master_copy: 4 {:?}", master_copy[4].len());
    // println!("master_copy: 5 {:?}", master_copy[5].len());
    // println!("master_copy: 6 {:?}", master_copy[6].len());
    // println!("master_copy: 7 {:?}", master_copy[7].len());
    // println!("master_copy: 8 {:?}", master_copy[8].len());
    // println!("master_copy: 9 {:?}", master_copy[9].len());
    // println!("master_copy: 10 {:?}", master_copy[10].len());
    // println!("master_copy: 11 {:?}", master_copy[11].len());
    // println!("master_copy: 12 {:?}", master_copy[12].len());
    
    // Construct the parity shards
    r.encode(&mut master_copy).unwrap();

    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = master_copy.iter().cloned().map(Some).collect();
    
    // END TIME
    let batch_chunk_creationg_time = start_1.start.elapsed().as_millis();
    println!("batch chunk creation time :: {:?}", batch_chunk_creationg_time);

    let re_creation = Instant::now(); 
    
    // We can remove up to few shards, which may be data or parity shards
    // DESTROYING FEW SHARDS, DATA OR PARITY SHARDS
    shards[1] = None;
    shards[3] = None;
    shards[5] = None;
    //shards[7] = None;

    // TRY TO RECONSTRUCT SHARDS
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
    //
    // let output_src = str::from_utf8(&result[0]).unwrap();    // BYTES TO ORIGINAL DATA
    //
    // //println!("O/T : {}", output_src);

    //
    let end   = time::precise_time_ns();
    let time_taken = (end - start) as f64 / 1_000_000_000.0;
    
    //
    // println!("    time taken       : {}", time_taken);

    let re_creationg_time = re_creation.start.elapsed().as_millis();
    println!("batch chunk re- creation time :: {:?}", re_creationg_time);

    assert!(r.verify(&result).unwrap());
    //println!("master_copy: {:?}", master_copy);
    assert_eq!(master_copy, result);
    println!("Result (OK) ");
    Ok(())
}    
