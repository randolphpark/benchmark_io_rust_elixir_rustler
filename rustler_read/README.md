# Rustler Read

Binary file "lib/hibp_binary" with 517,238,891 records

```elixir
iex -S mix
iex> RustlerRead.bench()
```

Benchmark Result
```
Name                                       ips        average  deviation         median         99th %
fixed_index_rustler                    52.76 K       18.95 μs    ±25.47%          18 μs          40 μs
fixed_index_pread                      37.02 K       27.01 μs    ±34.63%          25 μs          66 μs
random_maker                           18.68 K       53.53 μs    ±22.81%          50 μs         109 μs
fixed_index_rustler_29_times           14.28 K       70.04 μs    ±17.44%          68 μs         134 μs
binary_search_ruslter_has_match        11.88 K       84.17 μs    ±21.09%          79 μs         163 μs
binary_search_ruslter_no_match_4       11.30 K       88.52 μs    ±17.40%          86 μs         168 μs
binary_search_ruslter_no_match_5       11.22 K       89.15 μs    ±17.30%          87 μs         170 μs
binary_search_ruslter_no_match_3       11.07 K       90.36 μs    ±19.36%          86 μs         173 μs
binary_search_ruslter_no_match_1       10.90 K       91.75 μs    ±19.13%          88 μs         179 μs
binary_search_ruslter_no_match_2       10.81 K       92.48 μs    ±19.77%          88 μs      179.26 μs
random_index_pread                      1.36 K      733.34 μs    ±12.92%         705 μs     1172.92 μs
binary_search_ruslter_random            1.15 K      869.02 μs    ±35.78%         909 μs     1590.05 μs
```

