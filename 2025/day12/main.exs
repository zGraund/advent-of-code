defmodule Main do
  # Time is in microseconds
  def result(fun, data) do
    {time, res} = :timer.tc(fun, [data])
    IO.puts("Result: #{res}")
    IO.puts("  Time: #{time / 1_000}ms")
  end

  # In the real input (and only in the real input), the area below the tree is either
  # much larger or much smaller than the total area of all the presents placed side by side.
  # This means the only thing we need to check in this final challenge is whether the available
  # area is greater than the total area required by the presents.
  def part1(data) do
    {_, trees} = String.split(data, "\n\n", trim: true) |> Enum.split(-1)

    # All shapes are 3x3
    shapes_area = 9

    trees
    |> hd()
    |> String.split("\n", trim: true)
    |> Enum.count(fn line ->
      {area, shapes} =
        Regex.scan(~r/\d+/, line)
        |> Enum.map(&(hd(&1) |> String.to_integer()))
        |> Enum.split(2)

      Enum.product(area) >= Enum.sum(shapes) * shapes_area
    end)
  end
end

data = File.read!("day12/input.txt")
Main.result(&Main.part1/1, data)
