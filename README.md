# erasure-chunk

### TEST


**N **	:	Chunk created from source file
**K**					: Parity Chunks
**FILENAME**	:	Transactions file ( Input )


###### Cargo Run :

```shell
%/> cargo run 8 5 transactions_4pt5mb_20k.json
Arguments : Args { inner: ["./target/debug/erasure-chunk", "8", "5", "transactions_4pt5mb_20k.json"] }
ARGUMENT :- N = 8  K = 5 File Name = "transactions_4pt5mb_20k.json"
result: 13
    time taken       : 0.229442995
```

###### Cargo Build & Run

```shell
%/> cargo build
Compiling erasure_code v0.1.0 (/Users/erasure_code)
Finished dev [unoptimized + debuginfo] target(s) in 1.67s

%/> ./target/debug/erasure-chunk 8 5 transactions_4pt5mb_20k.json

Arguments : Args { inner: ["./target/debug/erasure-chunk", "8", "5", "transactions_4pt5mb_20k.json"] }
ARGUMENT :- N = 8  K = 5 File Name = "transactions_4pt5mb_20k.json"

result: 13
    time taken       : 0.342490104
```


