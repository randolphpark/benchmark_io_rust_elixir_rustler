# Rust Read

Binary file "lib/database_fixture" with 10 records

```rust
cargo bench
```

Benchmark Result
```
test benchmark_read_30_time           ... bench:     340,469 ns/iter (+/- 27,017)
test benchmark_read_30_time_with_seek ... bench:      63,971 ns/iter (+/- 4,535)
test benchmark_read_single            ... bench:      11,204 ns/iter (+/- 544)
test binary_search                    ... bench:      23,185 ns/iter (+/- 767)
```

