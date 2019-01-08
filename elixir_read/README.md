# Elixir Read

Binary file "lib/database_fixture" with 10 records

```elixir
iex -S mix
iex> ElixirRead.bench()
```


Benchmark Result
```
Name                                   ips        average  deviation         median         99th %
benchmark_pread_fixed_index        38.89 K       25.72 μs    ±28.37%          25 μs          59 μs
benchmark_pread_random_index       38.11 K       26.24 μs    ±33.25%          25 μs          61 μs
```

