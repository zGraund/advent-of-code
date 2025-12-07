defmodule Main do
  # Time is in microseconds
  def result(fun, data) do
    {time, res} = :timer.tc(fun, [data])
    IO.puts("Result: #{res}")
    IO.puts("  Time: #{time / 1_000}ms")
  end

  def to_int(list) do
    list |> Enum.map(&String.to_integer/1)
  end

  def part1(data) do
    data
    |> String.split("\n", trim: true)
    |> Enum.map(&String.split(&1, ~r/\s+/, trim: true))
    # Zip to transpose the input and get the columns instead of rows
    |> Enum.zip_with(&Enum.reverse/1)
    |> Enum.reduce(0, fn [head | tail], acc ->
      acc +
        cond do
          head == "+" -> tail |> to_int() |> Enum.sum()
          head == "*" -> tail |> to_int() |> Enum.product()
          true -> :invalid
        end
    end)
  end

  def zip_and_join(nums) do
    nums
    |> Enum.zip_with(&Enum.reverse/1)
    |> Enum.map(fn num ->
      num
      |> Enum.join()
      |> String.trim()
      |> String.to_integer()
    end)
  end

  # Don't reverse the result because the string was reversed before
  # calling this function
  def find_groups("", res), do: res

  # Find the length of each group of operations using the 
  # line with all symbols since they are always at the
  # start of the group
  def find_groups(str, res) do
    {ind, _} = Regex.run(~r/\+|\*/, str, return: :index) |> hd()
    # Split at +2 to skip the empty space between each group
    {_, tail} = String.split_at(str, ind + 2)
    find_groups(tail, [ind + 1 | res])
  end

  def part2(data) do
    lines = data |> String.split("\n", trim: true)

    # Reverse the string so it's easier to split
    groups = find_groups(lines |> List.last() |> String.reverse(), [])

    lines
    |> Enum.map(fn line ->
      # Split the line into groups using the groups length
      groups
      |> Enum.map_reduce(line, fn len, str ->
        {group, rest} = String.split_at(str, len)
        # Slice to skip the empty space between groups
        {group, String.slice(rest, 1..-1//1)}
      end)
      |> elem(0)
      |> Enum.map(&String.graphemes/1)
    end)
    # Same as part 1 transpose to get the columns
    |> Enum.zip_with(&Enum.reverse/1)
    |> Enum.reduce(0, fn [head | tail], acc ->
      acc +
        cond do
          # And transpose again to get the numbers ordered by columns
          hd(head) == "+" -> tail |> zip_and_join() |> Enum.sum()
          hd(head) == "*" -> tail |> zip_and_join() |> Enum.product()
          true -> :invalid
        end
    end)
  end
end

data = File.read!("day06/input.txt")

Main.result(&Main.part1/1, data)
Main.result(&Main.part2/1, data)
