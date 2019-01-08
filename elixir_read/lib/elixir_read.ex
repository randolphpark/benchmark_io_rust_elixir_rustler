defmodule ElixirRead do
  @database_path "lib/database_fixture"

  def benchmark_pread() do
    {:ok, file} = :file.open(@database_path, [:binary, :read])
    Benchee.run(%{"benchmark_pread" => fn -> ElixirRead.read_file(file, 1) end})
  end

  def read_file(file, offset) do
    :file.pread(file, offset*14, 14)
  end
end
