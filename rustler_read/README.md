# Rustler Read

Binary file "lib/database_fixture" with 10 records

```elixir
iex -S mix
iex> RustlerRead.bench()
```

Benchmark Result
```
Name                                       ips        average  deviation         median         99th %
fixed_index_rustler_single             50.07 K       19.97 μs    ±31.68%          19 μs          51 μs
fixed_index_pread                      35.57 K       28.11 μs    ±46.07%          25 μs          91 μs
random_maker                           17.74 K       56.35 μs    ±23.24%          54 μs         117 μs
fixed_index_rustler_29                 13.33 K       75.02 μs    ±20.50%          71 μs         150 μs
binary_search_ruslter_has_match        12.39 K       80.68 μs    ±20.67%          77 μs         160 μs
binary_search_ruslter_no_match_2       11.34 K       88.17 μs    ±15.92%          86 μs      164.80 μs
binary_search_ruslter_no_match_5       11.31 K       88.40 μs    ±17.41%          86 μs         169 μs
binary_search_ruslter_no_match_3       10.97 K       91.16 μs    ±21.88%          87 μs         187 μs
binary_search_ruslter_no_match_1       10.85 K       92.19 μs    ±18.94%          88 μs         177 μs
binary_search_ruslter_no_match_4       10.75 K       93.05 μs    ±21.22%          89 μs         190 μs
random_index_pread                      1.32 K      758.06 μs    ±27.62%         667 μs        1590 μs
binary_search_ruslter_random            0.99 K     1008.59 μs    ±34.19%        1058 μs     1777.10 μs```
```
