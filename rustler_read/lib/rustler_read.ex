defmodule RustlerRead do
  def read_file(file, offset) do
    :file.pread(file, offset*14, 14)
  end

  def multiple_pread(file) do
    [1,2,3,4,5,6,7,8,9,0,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8]
    |> Enum.each(fn num -> :file.pread(file, num*14, 14) end)
  end

  def bench() do
    {:ok, file} = :file.open("native/nifreader/database_fixture", [:binary, :read])
    Benchee.run(
      %{
        "fixed_index_rustler_single" => fn -> NifReader.seek_line(1) end,
        "fixed_index_rustler_29" => fn -> NifReader.seek_29_times(1) end,
        "binary_search_ruslter_single" => fn -> pwn() end,
        "binary_search_ruslter_random" => fn -> pwn_random() end,
        "fixed_index_pread" => fn -> read_file(file, 1) end,
        "random_index_pread" => fn -> multiple_pread(file) end,
        "random_maker" => fn -> random_maker() end,
        }
    )
  end

  def pwn() do
    target = [221, 93, 88, 98, 146, 95, 31, 149, 60, 171];
    NifReader.pwn_check(target)
  end

  def pwn_random() do
    list = 0..256
    target = Enum.take_random(list, 10)
    NifReader.pwn_check(target)
  end

  def random_maker do
    list = 0..256
    Enum.take_random(list, 10)
  end
end

defmodule NifReader do
    use Rustler, otp_app: :rustler_read, crate: "nifreader"

    # When your NIF is loaded, it will override this function.
    def seek_line(_a), do: :erlang.nif_error(:nif_not_loaded)
    def seek_29_times(_a), do: :erlang.nif_error(:nif_not_loaded)
    def pwn_check(_a), do: :erlang.nif_error(:nif_not_loaded)
end
