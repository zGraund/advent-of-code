defmodule Main do
  import Bitwise
  @epsilon 1.0e-9

  # Time is in microseconds
  def result(fun, data) do
    {time, res} = :timer.tc(fun, [data])
    IO.puts("Result: #{res}")
    IO.puts("  Time: #{time / 1_000}ms")
  end

  def solve(target, queue, buttons, visited) do
    {item, queue} = :queue.out(queue)
    {state, last, presses} = elem(item, 1)

    if MapSet.member?(visited, {state, last}) do
      solve(target, queue, buttons, visited)
    else
      visited = MapSet.put(visited, {state, last})

      queue =
        Enum.reduce_while(buttons, queue, fn button, q ->
          if button == last do
            {:cont, q}
          else
            state = bxor(state, button)
            q = :queue.in({state, button, presses + 1}, q)

            if state == target, do: {:halt, q}, else: {:cont, q}
          end
        end)

      peek = :queue.peek_r(queue) |> elem(1)

      if elem(peek, 0) == target,
        do: elem(peek, 2),
        else: solve(target, queue, buttons, visited)
    end
  end

  # Basic BFS implementation with a queue. The current state, each button
  # and the latest button used are encoded with a bitmask
  def part1(data) do
    data
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      wanted =
        Regex.run(~r/\[([.#]+)\]/, line, capture: :all_but_first)
        |> hd()
        |> String.replace([".", "#"], fn
          "." -> "0"
          "#" -> "1"
        end)
        |> String.reverse()
        |> String.to_integer(2)

      buttons =
        Regex.scan(~r/\(([\d,]+)\)/, line, capture: :all_but_first)
        |> Enum.map(fn button ->
          String.split(hd(button), ",", trim: true)
          |> Enum.reduce(0, fn digit, mask ->
            bxor(mask, 1 <<< String.to_integer(digit))
          end)
        end)

      # A queue node is { current_state, latest_button_used, presses_count }
      q = :queue.from_list([{0, 0, 0}])

      solve(wanted, q, buttons, MapSet.new())
    end)
    |> Enum.sum()
  end

  def gaussian_elimination(matrix, row_ind \\ 0)
  def gaussian_elimination(matrix, row_ind) when row_ind >= length(matrix), do: matrix

  # https://en.wikipedia.org/wiki/Gaussian_elimination
  # Find the pivot in each row and eliminate its column from all other rows
  # to get the matrix in reduced row echelon form, sorting is not necessary
  def gaussian_elimination(matrix, row_ind) do
    subtrahend = Enum.at(matrix, row_ind)

    # Ignore augmented column when searching for pivot
    pivot_col = subtrahend |> Enum.drop(-1) |> Enum.find_index(&(&1 != 0))

    # Skip a row if it's only 0
    if pivot_col == nil do
      gaussian_elimination(matrix, row_ind + 1)
    else
      pivot_val = Enum.at(subtrahend, pivot_col)

      # Normalize pivot row
      normalized = Enum.map(subtrahend, fn v -> v / pivot_val end)

      matrix =
        matrix
        |> Enum.with_index()
        |> Enum.map(fn {row, ind} ->
          # Skip rows that don't have a value in the same column as the pivot
          # and update the subtrahend row with it's normalized version
          cond do
            ind == row_ind ->
              normalized

            Enum.at(row, pivot_col) == 0 ->
              row

            true ->
              factor = Enum.at(row, pivot_col)
              Enum.zip_with(row, normalized, &(&1 - factor * &2))
          end
        end)

      gaussian_elimination(matrix, row_ind + 1)
    end
  end

  # Get the original matrix (A * x = b) and extract a list of equations in
  # reverse order that solve each pivot value using the free variables and b,
  # the first value is b and the others are a tuple with the index of the button
  # and the value from the matrix
  # 
  # [ 1 0 0 1 0 -1 | 2 ] -> [2, {5, -1}, {3, 1}]
  #   0 1 2 3 4  5   6 <- index
  #
  # basically isolate the pivot from the equation:
  #
  # pivot          b      pivot            b
  # v              v      v                v
  # n0 + n3 - n5 = 2  ->  n0 = - n3 + n5 + 2
  #      ^    ^                  ^    ^
  #      free variables          free variables
  #
  def equations_matrix(matrix) do
    Enum.map(matrix, fn row ->
      len = length(row)

      row
      |> Enum.with_index()
      # Drop zeros until the pivot (RREF guarantees pivot is first non-zero)
      # and the pivot itself
      |> Enum.drop_while(&(elem(&1, 0) == 0))
      |> Enum.drop(1)
      |> Enum.reduce([], fn {val, ind}, acc ->
        cond do
          ind == len - 1 -> [val | acc]
          val == 0 -> acc
          true -> [{ind, val} | acc]
        end
      end)
    end)
  end

  # From the equations matrix get all free variable and the max
  # possible value that each can have (calculated using the joltage)
  # and return a list of tuple to use in the dfs
  def starting_status(equations, all_buttons, joltage) do
    max_presses =
      Enum.map(all_buttons, fn counters ->
        counters
        |> Enum.map(&Enum.at(joltage, &1))
        |> Enum.min()
      end)
      |> List.to_tuple()

    Enum.flat_map(equations, fn eq ->
      eq
      |> Enum.drop(1)
      |> Enum.map(fn {ind, _} -> {ind, elem(max_presses, ind)} end)
    end)
    |> Enum.uniq()
    |> List.to_tuple()
  end

  # Each equation is a list where the first element is the constant (the b in the A * x = b matrix)
  # and the other elements are the free variables, given each free variable value
  # we can calculate the pivot by subtracting the sum of the free variables from b
  #
  # fv = free_variable
  # [ b, {fv1_ind, fv1_val}, {fv2_ind, fv2_val}, ... ]
  #
  def is_valid(equations, status) do
    equations
    |> Enum.reduce_while(0, fn eq, sum ->
      pivot_val =
        eq
        |> Enum.drop(1)
        |> Enum.reduce(List.first(eq, 0), fn {matrix_ind, matrix_val}, diff ->
          {_, button_val} = Enum.find(status, fn {button_ind, _} -> matrix_ind == button_ind end)
          diff - button_val * matrix_val
        end)

      # If b is negative or not an int we discard it since the problem require
      # non negative integers for each variable, using @epsilon to avoid floating point errors
      if pivot_val < -@epsilon || abs(pivot_val - round(pivot_val)) >= @epsilon do
        {:halt, :invalid}
      else
        {:cont, sum + pivot_val}
      end
    end)
  end

  # Again a simple dfs to iterate over each permutation of the free variables, that
  # after the reduction with the Gaussian elimination are manageable
  # returns the min value found
  def solve2(equations, buttons, ind \\ 0, status \\ [], minn \\ :infinity) do
    if ind >= tuple_size(buttons) do
      eq_sum = is_valid(equations, status)

      if eq_sum == :invalid do
        minn
      else
        min(eq_sum + Enum.sum_by(status, &elem(&1, 1)), minn) |> trunc()
      end
    else
      {button_index, button_max} = elem(buttons, ind)

      Enum.reduce(0..button_max, minn, fn times, minn ->
        status = [{button_index, times} | status]
        solve2(equations, buttons, ind + 1, status, minn)
      end)
    end
  end

  def part2(data) do
    data
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      all_buttons =
        Regex.scan(~r/\(([\d,]+)\)/, line, capture: :all_but_first)
        |> Enum.map(fn button ->
          String.split(hd(button), ",", trim: true)
          |> Enum.map(&String.to_integer/1)
        end)

      joltage =
        Regex.run(~r/\{([\d,]+)\}/, line, capture: :all_but_first)
        |> Enum.flat_map(fn joltage ->
          String.split(joltage, ",", trim: true)
          |> Enum.map(&String.to_integer/1)
        end)

      matrix =
        joltage
        |> Enum.with_index()
        |> Enum.map(fn {counter, ind} ->
          all_buttons
          |> Enum.map(fn button ->
            if Enum.member?(button, ind), do: 1, else: 0
          end)
          |> Enum.concat([counter])
        end)
        |> gaussian_elimination()

      equations = equations_matrix(matrix)
      status = starting_status(equations, all_buttons, joltage)

      solve2(equations, status)
    end)
    |> Enum.sum()
  end
end

data = File.read!("day10/input.txt")
Main.result(&Main.part1/1, data)
Main.result(&Main.part2/1, data)
