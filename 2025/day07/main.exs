defmodule Main do
  # Time is in microseconds
  def result(fun, data) do
    {time, res} = :timer.tc(fun, [data])
    IO.puts("Result: #{res}")
    IO.puts("  Time: #{time / 1_000}ms")
  end

  # Standard flood-fill DFS to count each splitter that the beam encounters
  def dfs(map, curr, height) do
    {row, col} = curr

    cond do
      row >= height ->
        map

      Map.has_key?(map, curr) ->
        if !Map.get(map, curr) do
          map
          |> Map.put(curr, true)
          |> dfs({row, col + 1}, height)
          |> dfs({row, col - 1}, height)
        else
          map
        end

      true ->
        dfs(map, {row + 1, col}, height)
    end
  end

  def part1(data) do
    lines = data |> String.split("\n", trim: true)

    map =
      for {line, row} <- lines |> Enum.with_index(),
          {char, col} <- line |> String.graphemes() |> Enum.with_index(),
          char == "^",
          into: %{} do
        {{row, col}, false}
      end

    start_col = lines |> List.first() |> :binary.match("S") |> elem(0)

    dfs(map, {0, start_col}, length(lines)) |> Enum.count(fn {_, v} -> v == true end)
  end

  # Memoized DFS that count all paths to reach each leaf
  #                               ^                  ^
  #                           timelines             end
  def dfs_p2(curr, height, map) do
    {row, col} = curr

    cond do
      row >= height ->
        {1, map}

      Map.has_key?(map, curr) ->
        partial = Map.get(map, curr)

        if partial <= 0 do
          {partial_left, map} = dfs_p2({row, col - 1}, height, map)
          {partial_right, map} = dfs_p2({row, col + 1}, height, map)

          tot = partial_right + partial_left
          {tot, Map.put(map, curr, tot)}
        else
          {partial, map}
        end

      true ->
        dfs_p2({row + 1, col}, height, map)
    end
  end

  def part2(data) do
    lines = data |> String.split("\n", trim: true)

    map =
      for {line, row} <- lines |> Enum.with_index(),
          {char, col} <- line |> String.graphemes() |> Enum.with_index(),
          char == "^",
          into: %{} do
        {{row, col}, 0}
      end

    start_col = lines |> List.first() |> :binary.match("S") |> elem(0)

    dfs_p2({0, start_col}, length(lines), map) |> elem(0)
  end
end

data = File.read!("day07/input.txt")
Main.result(&Main.part1/1, data)
Main.result(&Main.part2/1, data)
