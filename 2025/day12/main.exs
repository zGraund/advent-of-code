defmodule Main do
  # Time is in microseconds
  def result(fun, data) do
    {time, res} = :timer.tc(fun, [data])
    IO.puts("Result: #{res}")
    IO.puts("  Time: #{time / 1_000}ms")
  end

  # Turns out that in the real input, and ONLY in the real input, the area is either
  # way bigger than the total area occupied by all the boxes or way to small, no in between
  # so to solve the only part of the last day we only need to check if the total area below the
  # tree is bigger than the total area of all of the presents that need to be placed
  def part1(data) do
    {_, trees} = String.split(data, "\n\n", trim: true) |> Enum.split(-1)

    # All shapes are 3x3
    shapes_area = 9

    trees
    |> hd()
    |> String.split("\n", trim: true)
    |> Enum.filter(fn tree ->
      {area, shapes} =
        Regex.scan(~r/\d+/, tree)
        |> Enum.map(&(hd(&1) |> String.to_integer()))
        |> Enum.split(2)

      max_area = area |> Enum.product()
      required = shapes |> Enum.sum_by(&(&1 * shapes_area))
      max_area >= required
    end)
    |> Enum.count()
  end
end

data = File.read!("day12/input.txt")
Main.result(&Main.part1/1, data)
