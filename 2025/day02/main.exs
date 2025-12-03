require Integer

defmodule Main do
  def part1(data) do
    data
    |> String.split([",", "\n"], trim: true)
    |> Enum.flat_map(fn pair ->
      [start, stop] = pair |> String.split("-") |> Enum.map(&String.to_integer/1)
      start..stop
    end)
    |> Enum.filter(fn num ->
      # Get the number of digits in the current number
      len = trunc(:math.log10(num) + 1)

      # Odd numbers can't be invalid ids
      if Integer.is_even(len) do
        # The number of digits to split the number in 2
        digits = Integer.pow(10, Integer.floor_div(len, 2))
        # Here we basically split the the number in 2 and check if
        # both half are the same
        Integer.floor_div(num, digits) == rem(num, digits)
      else
        false
      end
    end)
    |> Enum.sum()
    |> IO.puts()
  end

  def part2(data) do
    data
    |> String.split([",", "\n"], trim: true)
    |> Enum.flat_map(fn pair ->
      [start, stop] = pair |> String.split("-") |> Enum.map(&String.to_integer/1)
      start..stop
    end)
    |> Enum.filter(fn num ->
      # Ignore the edge case of single digits numbers
      if num <= 9 do
        false
      else
        digits = Integer.digits(num)
        len = length(digits)

        # Divide the number into chunks and check if they are all equal
        # we can limit to len / 2 since that's the least amount of groups
        # possible
        Enum.any?(1..Integer.floor_div(len, 2)//1, fn size ->
          # Skip numbers not divisible for the current size
          # as they cannot be an invalid id
          if rem(len, size) == 0 do
            digits
            |> Enum.chunk_every(size)
            |> Enum.map(&Integer.undigits/1)
            |> MapSet.new()
            |> MapSet.size() ==
              1
          end
        end)
      end
    end)
    |> Enum.sum()
    |> IO.puts()
  end
end

data = File.read!("day02/input.txt")
Main.part1(data)
Main.part2(data)
