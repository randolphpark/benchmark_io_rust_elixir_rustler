defmodule RustlerReadTest do
  use ExUnit.Case
  doctest RustlerRead

  test "greets the world" do
    assert RustlerRead.hello() == :world
  end
end
