defmodule Main do
  # Time is in microseconds
  def result(fun, data) do
    {time, res} = :timer.tc(fun, data)
    IO.puts("Result: #{res}")
    IO.puts("  Time: #{time / 1_000}ms")
  end

  def generate_pairs(data) do
    boxes =
      data
      |> String.split("\n", trim: true)
      |> Enum.with_index()
      |> Enum.map(fn {str, ind} ->
        {str |> String.split(",") |> Enum.map(&String.to_integer/1) |> List.to_tuple(), ind}
      end)

    # Generate each connection
    pairs =
      for {first, ind} <- boxes,
          {second, _} <- boxes |> Enum.drop(ind + 1) do
        {first, second}
      end
      # Sort by Euclidean distance, the formula is the Pythagorean theorem
      # using all 3 points of each box
      |> Enum.sort_by(fn {{x1, y1, z1}, {x2, y2, z2}} ->
        :math.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2 + (z2 - z1) ** 2)
      end)

    {pairs, length(boxes)}
  end

  def insert_node(map, curr, node) do
    Map.update(map, curr, MapSet.new([node]), fn set ->
      MapSet.put(set, node)
    end)
  end

  def count_nodes(visited, graph, curr) do
    if MapSet.member?(visited, curr) do
      visited
    else
      graph
      |> Map.get(curr)
      |> Enum.reduce(MapSet.put(visited, curr), fn node, acc ->
        count_nodes(acc, graph, node)
      end)
    end
  end

  def part1(connections, len) do
    adjacency =
      connections
      # 10 connections with test input and 1000 with real input
      |> Enum.take(if len <= 20, do: 10, else: 1000)
      # Make an adjacency map to represent the graphs
      |> Enum.reduce(Map.new(), fn {first, second}, graph ->
        graph |> insert_node(first, second) |> insert_node(second, first)
      end)

    {_, sizes} =
      adjacency
      |> Enum.reduce({MapSet.new(), []}, fn {node, _}, {visited, acc} ->
        if MapSet.member?(visited, node) do
          {visited, acc}
        else
          new_visited = count_nodes(visited, adjacency, node)
          count = MapSet.size(new_visited) - MapSet.size(visited)
          {new_visited, [count | acc]}
        end
      end)

    sizes
    |> Enum.sort(:desc)
    |> Enum.take(3)
    |> Enum.product()
  end

  # Very naive and ugly brute force that finishes in ~10s.
  # Populate the adjacency map one connection at a time and
  # check whether any connected component reaches the input size.
  # This could have been easily solved with union-find.
  def part2(connections, len) do
    {first, second} =
      connections
      |> Enum.reduce_while(Map.new(), fn {first, second}, graphs ->
        graphs = graphs |> insert_node(first, second) |> insert_node(second, first)

        count =
          count_nodes(MapSet.new(), graphs, Enum.random(graphs) |> elem(0))
          |> MapSet.size()

        if count != len do
          {:cont, graphs}
        else
          {:halt, {first, second}}
        end
      end)

    elem(first, 0) * elem(second, 0)
  end
end

data = File.read!("day08/input.txt")
{time, {connections, input_len}} = :timer.tc(&Main.generate_pairs/1, [data])
IO.puts("Connections: Ok!")
IO.puts("  Took: #{time / 1000}ms")
Main.result(&Main.part1/2, [connections, input_len])
Main.result(&Main.part2/2, [connections, input_len])
