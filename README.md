# hash-comparison

Cmd:

```sh
 RUSTFLAGS="-C target-feature=+aes,+ssse3,+sse4.2" cargo bench --target=x86_64-unknown-linux-musl
```

| Alg       |     License     |                       Problems |
| :-------- | :-------------: | -----------------------------: |
| xxhash    |     BSL-1.0     | DiffDist bit 7 w. 36 bits, BIC |
| Ahash     | MIT, Apache-2.0 |                           None |
| Wyhash    | MIT, Apache-2.0 |                           None |
| Seahash   |       MIT       |             PerlinNoise, !msvc |
| Metrohash |       MIT       |              UB, LongNeighbors |
| Highway   |       MIT       |                           None |
| Fxhash    |       ???       |                            ??? |
| CityHash  | MIT, Apache-2.0 |               Sparse, TwoBytes |

## All Algs

![hash plot](./criterion/hash64/report/lines.svg)

<!-- <img src="./criterion/hash64/report/lines.svg"> -->

![hash violin](./criterion/hash64/report/violin.svg)

<!-- <img src="./criterion/hash64/report/violin.svg"> -->

## All Algs (4B, 8B, 16B)

<!-- ![hash plot]("./criterion/hash64 (4B, 8B, 16B)/report/lines.svg") -->

<!-- ![hash violin]("./criterion/hash64 (4B, 8B, 16B)/report/violin.svg") -->

<img src="./criterion/hash64 (4B, 8B, 16B)/report/lines.svg">

<img src="./criterion/hash64 (4B, 8B, 16B)/report/violin.svg">

## AHash, Fxhash, Metrohash, Wyhash (4B, 8B, 16B)

<!-- ![hash plot]("./criterion/hash64 (4B, 8B, 16B) v2.0/report/lines.svg") -->

<!-- ![hash violin]("./criterion/hash64 (4B, 8B, 16B) v2.0/report/violin.svg") -->

<img src="./criterion/hash64 (4B, 8B, 16B) v2.0/report/lines.svg">

<img src="./criterion/hash64 (4B, 8B, 16B) v2.0/report/violin.svg">

## AHash (+ssse3, +aes) vs Ahash

```sh
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

ahash/ahash             time:   [16.351 ns 16.372 ns 16.395 ns]
                        thrpt:  [232.67 MiB/s 233.01 MiB/s 233.30 MiB/s]
                 change:
                        time:   [+4.2846% +4.7066% +5.1588%] (p = 0.00 < 0.05)
                        thrpt:  [-4.9057% -4.4951% -4.1085%]
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe
ahash/ahash #2          time:   [16.339 ns 16.353 ns 16.368 ns]
                        thrpt:  [466.12 MiB/s 466.56 MiB/s 466.95 MiB/s]
                 change:
                        time:   [+4.5496% +5.0921% +5.7024%] (p = 0.00 < 0.05)
                        thrpt:  [-5.3948% -4.8454% -4.3516%]
                        Performance has regressed.
Found 18 outliers among 100 measurements (18.00%)
  18 (18.00%) high severe
ahash/ahash #3          time:   [15.681 ns 15.696 ns 15.712 ns]
                        thrpt:  [971.14 MiB/s 972.15 MiB/s 973.05 MiB/s]
                 change:
                        time:   [+0.9679% +1.4990% +2.0145%] (p = 0.00 < 0.05)
                        thrpt:  [-1.9747% -1.4769% -0.9586%]
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
ahash/ahash #4          time:   [23.916 ns 23.941 ns 23.969 ns]
                        thrpt:  [9.9470 GiB/s 9.9585 GiB/s 9.9689 GiB/s]
                 change:
                        time:   [-23.666% -23.105% -22.556%] (p = 0.00 < 0.05)
                        thrpt:  [+29.126% +30.047% +31.003%]
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe
ahash/ahash #5          time:   [37.161 ns 37.191 ns 37.226 ns]
                        thrpt:  [25.618 GiB/s 25.642 GiB/s 25.664 GiB/s]
                 change:
                        time:   [-53.274% -52.919% -52.577%] (p = 0.00 < 0.05)
                        thrpt:  [+110.87% +112.40% +114.01%]
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe
ahash/ahash #6          time:   [96.132 ns 96.243 ns 96.363 ns]
                        thrpt:  [39.587 GiB/s 39.636 GiB/s 39.682 GiB/s]
                 change:
                        time:   [-64.944% -64.690% -64.454%] (p = 0.00 < 0.05)
                        thrpt:  [+181.33% +183.20% +185.26%]
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
ahash/ahash #7          time:   [322.28 ns 322.66 ns 323.10 ns]
                        thrpt:  [47.226 GiB/s 47.291 GiB/s 47.346 GiB/s]
                 change:
                        time:   [-68.823% -68.647% -68.478%] (p = 0.00 < 0.05)
                        thrpt:  [+217.23% +218.95% +220.75%]
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) high mild
  14 (14.00%) high severe
```
