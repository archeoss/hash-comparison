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

![hash plot]("./criterion/hash64 (4B, 8B, 16B)/report/lines.svg")

![hash violin]("./criterion/hash64 (4B, 8B, 16B)/report/violin.svg")

## AHash, Fxhash, Metrohash, Wyhash (4B, 8B, 16B)

![hash plot]("./criterion/hash64 (4B, 8B, 16B) v2.0/report/lines.svg")

![hash violin]("./criterion/hash64 (4B, 8B, 16B) v2.0/report/violin.svg")

## AHash (+ssse3, +aes) (4B, 8B, 16B)

![hash plot]("./criterion/ahash/ahash (+aes +ssse3)/report/lines.svg")

![hash violin]("./criterion/ahash/ahash (+aes +ssse3)/report/violin.svg")

## AHash (4B, 8B, 16B)

![hash plot](./criterion/ahash/ahash/report/lines.svg)

![hash violin](./criterion/ahash/ahash/report/violin.svg)
