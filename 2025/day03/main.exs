defmodule Main do
  def part1(data) do
    data
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      nums = String.graphemes(line) |> Enum.map(&String.to_integer/1)

      nums
      |> Enum.with_index()
      |> Enum.flat_map(fn {first, ind} ->
        nums
        |> Enum.drop(ind + 1)
        |> Enum.map(fn second ->
          first * 10 + second
        end)
      end)
      |> Enum.max()
    end)
    |> Enum.sum()
    |> IO.puts()
  end

  def find_candidate(_list, _start, -1, result), do: Enum.reverse(result)

  # In the range of: last digit chosen - last valid digit, search for the biggest
  # number as this guarantees that the result is the maximum possible.
  def find_candidate(list, start, remaining, result) do
    {number, index} =
      list
      |> Enum.slice(start, length(list) - remaining - start)
      |> Enum.max_by(fn {n, _i} -> n end)

    find_candidate(list, index + 1, remaining - 1, [number | result])
  end

  def part2(data) do
    data
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      line
      |> String.graphemes()
      |> Enum.map(&String.to_integer/1)
      |> Enum.with_index()
      # The 11 magic number is: number of batteries needed - 1
      |> find_candidate(0, 11, [])
      |> Integer.undigits()
    end)
    |> Enum.sum()
    |> IO.puts()
  end
end

data = File.read!("day03/input.txt")
Main.part1(data)
Main.part2(data)
