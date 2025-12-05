defmodule Main do
  def part1(data) do
    data
    |> String.split("\n\n", trim: true)
    |> List.to_tuple()
    |> then(fn {raw_ranges, raw_ids} ->
      ranges =
        raw_ranges
        |> String.split("\n")
        |> Enum.map(fn line ->
          line
          |> String.split("-")
          |> Enum.map(&String.to_integer/1)
          |> List.to_tuple()
        end)

      # Simple linear search: each id over each range
      raw_ids
      |> String.split("\n", trim: true)
      |> Enum.map(&String.to_integer/1)
      |> Enum.count(fn id ->
        ranges |> Enum.any?(fn {low, high} -> id >= low && id <= high end)
      end)
      |> IO.puts()
    end)
  end

  def part2(data) do
    data
    |> String.split("\n\n", trim: true)
    |> hd()
    |> String.split("\n")
    |> Enum.map(fn line ->
      line
      |> String.split("-")
      |> Enum.map(&String.to_integer/1)
      |> List.to_tuple()
    end)
    # Sort ranges by their start value
    |> Enum.sort()
    # Iterate through sorted ranges to count non overlapping values.
    # For each range, check if it overlaps with the previous range
    # by checking if the current start value is bigger than the previous end value
    |> Enum.reduce({0, 0}, fn {low, high}, {prev_high, count} ->
      cond do
        # No overlap: Add the full size of the current range.
        # + 1 because ranges are inclusive
        low > prev_high -> {high, count + high - low + 1}
        # Partial overlap: Add only the non overlapping part.
        high > prev_high -> {high, count + high - prev_high}
        # Full overlap: Ignore current range.
        true -> {prev_high, count}
      end
    end)
    |> elem(1)
    |> IO.puts()
  end
end

data = File.read!("day05/input.txt")
Main.part1(data)
Main.part2(data)
