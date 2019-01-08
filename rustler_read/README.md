# Rustler Read

Binary file "lib/database_fixture" with 10 records

```elixir
iex -S mix
iex> RustlerRead.bench()
```

Benchmark Result
```
Name                                   ips        average  deviation         median         99th %
fixed_index_rustler_single         54.29 K       18.42 μs    ±21.58%          18 μs          37 μs
fixed_index_pread                  36.90 K       27.10 μs    ±41.48%          25 μs          84 μs
fixed_index_rustler_29             14.40 K       69.43 μs    ±14.66%          66 μs         124 μs
binary_search_ruslter_single       12.89 K       77.57 μs    ±15.16%          74 μs         144 μs
random_index_pread                  1.27 K      785.98 μs    ±29.95%         690 μs     1718.51 μs
binary_search_ruslter_random        1.14 K      874.65 μs    ±26.18%         885 μs     1457.94 μs
```
