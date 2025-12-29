defmodule Main do
  # Time is in microseconds
  def result(fun, data) do
    {time, res} = :timer.tc(fun, [data])
    IO.puts("Result: #{res}")
    IO.puts("  Time: #{time / 1_000}ms")
  end

  # NOTE: Day 11 (2025) has two different test inputs.
  #       This solution only works with the real input.

  def solve(graph, current \\ "you", count \\ 0) do
    if current == "out" do
      count + 1
    else
      graph
      |> Map.get(current)
      |> Enum.reduce(count, fn child, count ->
        solve(graph, child, count)
      end)
    end
  end

  def part1(data) do
    data
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      [parent | childs] = String.split(line, ": ", parts: 2)
      {parent, String.split(hd(childs), " ")}
    end)
    |> Map.new()
    |> solve()
  end

  # NOTE: The solution for part 2 makes some assumptions about the input structure:
  #
  #       1) Since the result must be finite, if there is a path fft -> dac,
  #          there cannot be a path dac -> fft (and vice versa).
  #          Otherwise, the graph would contain a cycle, which would create
  #          infinite paths such as:
  #          svr -> fft -> dac -> fft -> ... -> out
  #
  #       2) There must be at least one valid path: either fft -> dac OR dac -> fft.
  #          Otherwise, the total number of valid paths would be 0.
  #

  def solve2(graph, current, target, visited \\ Map.new(), count \\ 0) do
    cond do
      current == target ->
        {count + 1, visited}

      # If we reached out here it means that it's not the target and we can stop
      current == "out" ->
        {count, visited}

      Map.has_key?(visited, current) ->
        {count + Map.get(visited, current), visited}

      true ->
        {partial, visited} =
          graph
          |> Map.get(current)
          |> Enum.reduce({0, visited}, fn child, {partial, visited} ->
            solve2(graph, child, target, visited, partial)
          end)

        {count + partial, Map.put(visited, current, partial)}
    end
  end

  def part2(data) do
    graph =
      data
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        [parent | childs] = String.split(line, ": ", parts: 2)
        {parent, String.split(hd(childs), " ")}
      end)
      |> Map.new()

    # The solution itself is very simple:
    #
    # 1) Determine the order of fft and dac in the input.
    #    If the number of paths fft -> dac is 0, then, according to the assumptions above,
    #    all valid paths must follow dac -> fft.
    #    If it is greater than 0, there is no need to check the reverse order,
    #    as it would necessarily be 0.
    #
    # 2) Calculate the number of paths in each remaining sub-path.
    #
    # 3) Multiply the number of paths in all sub-paths together.

    f_to_d = solve2(graph, "fft", "dac") |> elem(0)

    if f_to_d > 0 do
      [
        f_to_d,
        solve2(graph, "svr", "fft") |> elem(0),
        solve2(graph, "dac", "out") |> elem(0)
      ]
    else
      [
        solve2(graph, "dac", "fft") |> elem(0),
        solve2(graph, "svr", "dac") |> elem(0),
        solve2(graph, "ftt", "out") |> elem(0)
      ]
    end
    |> Enum.product()
  end
end

data = File.read!("day11/input.txt")
Main.result(&Main.part1/1, data)
Main.result(&Main.part2/1, data)
