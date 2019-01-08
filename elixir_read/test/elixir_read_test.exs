defmodule ElixirReadTest do
  use ExUnit.Case
  doctest ElixirRead

  test "greets the world" do
    assert ElixirRead.hello() == :world
  end
end
