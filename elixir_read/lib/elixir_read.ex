defmodule ElixirRead do
  @database_path "lib/database_fixture"

  def bench() do
    {:ok, file} = :file.open(@database_path, [:binary, :read])
    Benchee.run(
      %{
        "pread_fixed_index" => fn -> ElixirRead.read_file(file, 1) end,
        "pread_random_index" => fn -> ElixirRead.read_file_random(file) end,
      }
    )
  end

  def read_file(file, offset) do
    :file.pread(file, offset*14, 14)
  end

  def read_file_random(file) do
    :file.pread(file, :rand.uniform(10)*14, 14)
  end
end
