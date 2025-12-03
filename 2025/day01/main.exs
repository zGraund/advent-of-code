defmodule Main do
  def part1(data) do
    {_start, result} =
      data
      |> String.split("\n", trim: true)
      |> Enum.reduce({50, 0}, fn line, {start, result} ->
        steps = line |> String.slice(1..-1) |> String.to_integer()

        start =
          if String.first(line) == "L" do
            Integer.mod(start - steps, 100)
          else
            Integer.mod(start + steps, 100)
          end

        result = if start == 0, do: result + 1, else: result

        {start, result}
      end)

    IO.puts(result)
  end

  def part2(data) do
    {_start, result} =
      data
      |> String.split("\n", trim: true)
      |> Enum.reduce({50, 0}, fn line, {start, result} ->
        num = line |> String.slice(1..-1) |> String.to_integer()
        steps = rem(num, 100)
        mult = div(num, 100)

        {start, result} =
          if String.first(line) == "L" do
            new_pos = Integer.mod(start - steps, 100)

            new_res =
              if((new_pos > start || new_pos == 0) && start != 0, do: 1, else: 0) + mult + result

            {new_pos, new_res}
          else
            new_pos = Integer.mod(start + steps, 100)

            new_res = if(new_pos < start, do: 1, else: 0) + mult + result

            {new_pos, new_res}
          end

        {start, result}
      end)

    IO.puts(result)
  end
end

data = File.read!("day01/input.txt")
Main.part1(data)
Main.part2(data)
