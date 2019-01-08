defmodule RustlerRead do
  def benchmark_pread() do
    {:ok, file} = :file.open("native/nifreader/database_fixture", [:binary, :read])
    Benchee.run(%{"benchmark_pread" => fn -> read_file(file, 0) end})
  end

  def read_file(file, offset) do
    [1,2,3,4,5,6,7,8,9,0,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8]
    |> Enum.each(fn num -> :file.pread(file, num*14, 14) end)
  end

  def benchmark_rustler() do
    Benchee.run(%{"benchmark_rustler" => fn -> NifReader.read_file(1) end})
  end

  def benchmark() do
    {:ok, file} = :file.open("native/nifreader/database_fixture", [:binary, :read])
    Benchee.run(
      %{
        "benchmark_rustler" => fn -> NifReader.read_file(1) end,
        "benchmark_pread" => fn -> read_file(file, 0) end
        }
    )
    
  end
end

defmodule NifReader do
    use Rustler, otp_app: :rustler_read, crate: "nifreader"

    # When your NIF is loaded, it will override this function.
    def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
    def return_string(), do: :erlang.nif_error(:nif_not_loaded)
    def return_list(), do: :erlang.nif_error(:nif_not_loaded)
    def read_file(_a), do: :erlang.nif_error(:nif_not_loaded)
end
