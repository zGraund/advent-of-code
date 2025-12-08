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

  def to_int_p2(list) do
    list
    |> Enum.map(&(&1 |> Enum.drop(-1) |> Enum.join() |> String.trim() |> String.to_integer()))
  end

  # Refactored part 2, much easier to read and with a single transpose of the entire input
  # instead of splitting into group and then trasnposing 2 times 
  def part2refactored(data) do
    data
    |> String.split("\n", trim: true)
    |> Enum.map(&String.graphemes/1)
    # Transpose input
    |> Enum.zip_with(&Function.identity/1)
    # Filter out empty lines and group the rest of the input
    |> Enum.chunk_while(
      [],
      fn line, acc ->
        if line |> Enum.any?(&(&1 != " ")) do
          {:cont, [line | acc]}
        else
          {:cont, Enum.reverse(acc), []}
        end
      end,
      fn acc -> {:cont, Enum.reverse(acc), []} end
    )
    # Grab the operation from the first line and process each group
    |> Enum.map(fn group ->
      op = group |> List.first() |> List.last()

      cond do
        op == "+" -> group |> to_int_p2() |> Enum.sum()
        op == "*" -> group |> to_int_p2() |> Enum.product()
        true -> :invalid
      end
    end)
    |> Enum.sum()
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
Main.result(&Main.part2refactored/1, data)
