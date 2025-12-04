defmodule Main do
  def get_neighbors({row, col}) do
    for row_offset <- -1..1,
        col_offset <- -1..1,
        {row_offset, col_offset} != {0, 0} do
      {row + row_offset, col + col_offset}
    end
  end

  def part1(data) do
    grid =
      data
      |> String.split("\n", trim: true)
      |> Enum.with_index()
      |> Enum.flat_map(fn {line, row} ->
        line
        |> String.graphemes()
        |> Enum.with_index()
        |> Enum.filter(fn {c, _x} -> c == "@" end)
        |> Enum.map(fn {_c, col} -> {row, col} end)
      end)
      |> MapSet.new()

    grid
    |> Enum.filter(fn coord ->
      coord |> get_neighbors() |> Enum.count(&MapSet.member?(grid, &1)) < 4
    end)
    |> Enum.count()
    |> IO.puts()
  end

  def remove_paper(grid, result) do
    valid =
      grid
      |> MapSet.filter(fn coord ->
        coord |> get_neighbors() |> Enum.count(&MapSet.member?(grid, &1)) < 4
      end)

    if MapSet.size(valid) > 0 do
      remove_paper(MapSet.difference(grid, valid), result + MapSet.size(valid))
    else
      result
    end
  end

  def part2(data) do
    data
    |> String.split("\n", trim: true)
    |> Enum.with_index()
    |> Enum.flat_map(fn {line, row} ->
      line
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.filter(fn {c, _x} -> c == "@" end)
      |> Enum.map(fn {_c, col} -> {row, col} end)
    end)
    |> MapSet.new()
    |> then(&remove_paper(&1, 0))
    |> IO.puts()
  end
end

data = File.read!("day04/input.txt")
Main.part1(data)
Main.part2(data)
