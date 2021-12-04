# erasure-chunk

### TEST


**N **	:	Chunk created from source file
**K**					: Parity Chunks
**FILENAME**	:	Transactions file ( Input )


###### Cargo Run :

####Output :

```shell
_______@______-______-Pro erasure-chunk % cargo run 8 5 transactions_4pt5mb_20k.json
Arguments : Args { inner: ["./target/debug/erasure-chunk", "8", "5", "transactions_4pt5mb_20k.json"] }
ARGUMENT :- N = 8  K = 5 File Name = "transactions_4pt5mb_20k.json"
result: 13
    time taken       : 0.229442995
```

###### Cargo Build & Run

####Output :

```shell
_______@______-______-Pro erasure-chunk % cargo build
Compiling guessing_game v0.1.0 (/Users/mahendrapanchal/Projects/rust/guessing_game)
warning: value assigned to `shards` is never read
  --> src/main.rs:18:17
   |
18 |         let mut shards = Vec::<Vec<u8>>::with_capacity(13);
   |                 ^^^^^^
...
82 |     let mut parity_shards = make_random_parity_shards!(per_chunk_size, k);                     // GENERATE 5 PARITY SHARD WITH 0's
   |                             --------------------------------------------- in this macro invocation
   |
   = note: `#[warn(unused_assignments)]` on by default
   = help: maybe it is overwritten before being read?
   = note: this warning originates in the macro `make_random_parity_shards` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 1.67s


_______@______-______-Pro erasure-chunk % ./target/debug/erasure-chunk 8 5 transactions_4pt5mb_20k.json
Arguments : Args { inner: ["./target/debug/erasure-chunk", "8", "5", "transactions_4pt5mb_20k.json"] }
ARGUMENT :- N = 8  K = 5 File Name = "transactions_4pt5mb_20k.json"
result: 13
    time taken       : 0.342490104
```


