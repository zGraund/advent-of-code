defmodule Main do
  # Time is in microseconds
  def result(fun, data) do
    {time, res} = :timer.tc(fun, [data])
    IO.puts("Result: #{res}")
    IO.puts("  Time: #{time / 1_000}ms")
  end

  def part1(data) do
    nums =
      data
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        line |> String.split(",") |> Enum.map(&String.to_integer/1) |> List.to_tuple()
      end)

    for {{x1, y1}, ind} <- nums |> Enum.with_index(),
        {x2, y2} <- nums |> Enum.drop(ind + 1) do
      (abs(x1 - x2) + 1) * (abs(y1 - y2) + 1)
    end
    |> Enum.max()
  end

  def intercept({x, y}, polygon_edge) when is_number(x) and is_number(y) do
    intercept({{x, y}, {x, y}}, polygon_edge)
  end

  def intercept({{fx1, fy1}, {fx2, fy2}}, {{sx1, sy1}, {sx2, sy2}}) do
    f_x_min = min(fx1, fx2)
    f_x_max = max(fx1, fx2)
    f_y_min = min(fy1, fy2)
    f_y_max = max(fy1, fy2)

    s_x_min = min(sx1, sx2)
    s_x_max = max(sx1, sx2)
    s_y_min = min(sy1, sy2)
    s_y_max = max(sy1, sy2)

    f_x_min <= s_x_max && f_x_max >= s_x_min && f_y_min <= s_y_max && f_y_max >= s_y_min
  end

  # Check if the first or last outer edge point intercept the inner edge
  # and remove it if needed, this fixes the problem of
  # the inner perimeter "turning into" the outer one
  def generate_edge("V", {{x, y1}, {_, y2}}, edges) do
    start = min(y1, y2)
    endd = max(y1, y2)
    start = if Enum.any?(edges, &intercept({x, start}, &1)), do: start + 1, else: start
    endd = if Enum.any?(edges, &intercept({x, endd}, &1)), do: endd - 1, else: endd
    {{x, start}, {x, endd}}
  end

  def generate_edge("H", {{x1, y}, {x2, _}}, edges) do
    start = min(x1, x2)
    endd = max(x1, x2)
    start = if Enum.any?(edges, &intercept({start, y}, &1)), do: start + 1, else: start
    endd = if Enum.any?(edges, &intercept({endd, y}, &1)), do: endd - 1, else: endd
    {{start, y}, {endd, y}}
  end

  def get_rectangle({x1, y1}, {x2, y2}) do
    [a, b, c, d] = [{x1, y1}, {x1, y2}, {x2, y2}, {x2, y1}]
    [{a, b}, {b, c}, {c, d}, {d, a}]
  end

  def intercept_any(first, second) do
    Enum.any?(first, fn first_edge ->
      Enum.any?(second, &intercept(first_edge, &1))
    end)
  end

  # This approach generates an outer border around the original polygon
  # and checks whether any candidate rectangle has edges that intersect it.
  # Rectangles that do are considered invalid and discarded.
  #
  # # = red tiles
  # X = green tiles
  # O = outer border
  #
  # .......OOOOO..
  # ......O#XXX#O.
  # ..OOOOOX...XO.
  # .O#XXXX#...XO.
  # .OX........XO.
  # .O#XXXXXX#.XO.
  # ..OOOOOOOX.XO.
  # ........O#X#O.
  # .........OOO..
  #
  def part2(data) do
    nums =
      data
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        line |> String.split(",") |> Enum.map(&String.to_integer/1) |> List.to_tuple()
      end)

    edges = Enum.zip(nums, tl(nums) ++ [hd(nums)])

    # Construct the outer edge of the polygon by moving
    # the edge 1 coordinate to the left, assuming that the
    # input corners are going clockwise
    outer_edges =
      Enum.map(edges, fn {{x1, y1}, {x2, y2}} ->
        cond do
          # Direction: down
          y2 > y1 -> generate_edge("V", {{x1 + 1, y1}, {x2 + 1, y2}}, edges)
          # Direction: up
          y2 < y1 -> generate_edge("V", {{x1 - 1, y1}, {x2 - 1, y2}}, edges)
          # Direction: right
          x2 > x1 -> generate_edge("H", {{x1, y1 - 1}, {x2, y2 - 1}}, edges)
          # Direction: left
          x2 < x1 -> generate_edge("H", {{x1, y1 + 1}, {x2, y2 + 1}}, edges)
        end
      end)

    for {{x1, y1}, ind} <- nums |> Enum.with_index(),
        {x2, y2} <- nums |> Enum.drop(ind + 1),
        not intercept_any(get_rectangle({x1, y1}, {x2, y2}), outer_edges) do
      (abs(x1 - x2) + 1) * (abs(y1 - y2) + 1)
    end
    |> Enum.max()
  end
end

data = File.read!("day09/input.txt")
Main.result(&Main.part1/1, data)
Main.result(&Main.part2/1, data)
