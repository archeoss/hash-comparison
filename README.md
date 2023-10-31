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

![hash plot](./criterion/hash64/report/lines.svg)

<!-- <img src="./criterion/hash64/report/lines.svg"> -->

![hash violin](./criterion/hash64/report/violin.svg)

<!-- <img src="./criterion/hash64/report/violin.svg"> -->
