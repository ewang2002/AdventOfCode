using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.RevisedSolutions
{
	using Set3Tuple = HashSet<(int x, int y, int z)>;
	using Set4Tuple = HashSet<(int x, int y, int z, int w)>;

	public class Day17Revised : BaseDay
	{
		private readonly char[][] _input;

		public Day17Revised(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(x => x.ToCharArray())
				.ToArray();

		public override string SolvePart1()
		{
			var set = new Set3Tuple();
			var amtCubes = 0;

			for (var i = 0; i < _input.Length; i++)
			for (var j = 0; j < _input[i].Length; j++)
			{
				if (_input[i][j] != '#')
					continue;
				set.Add((i, j, 0));
				amtCubes++;
			}

			var cyclesCompleted = 0;
			var min = -1;
			var max = _input.Length + 1;
			while (cyclesCompleted < 6)
			{
				var toAdd = new Set3Tuple();
				var toRemove = new Set3Tuple(); 
				for (var i = min; i < max; i++)
				for (var j = min; j < max; j++)
				for (var k = min; k < max; k++)
				{
					var neighbors = GetNeighbors(set, i, j, k);
					var activeNeighbors = neighbors.Count(x => x);

					if (set.Contains((i, j, k)) && activeNeighbors != 2 && activeNeighbors != 3)
					{
						toRemove.Add((i, j, k));
						amtCubes--;
						continue;
					}

					if (!set.Contains((i, j, k)) && activeNeighbors == 3)
					{
						toAdd.Add((i, j, k));
						amtCubes++;
					}
				}

				foreach (var add in toAdd)
					set.Add(add);

				foreach (var remove in toRemove)
					set.Remove(remove);

				min--;
				max++;
				cyclesCompleted++;
			}

			return amtCubes.ToString();
		}

		public override string SolvePart2()
		{
			var set = new Set4Tuple();
			var amtCubes = 0;

			for (var i = 0; i < _input.Length; i++)
			for (var j = 0; j < _input[i].Length; j++)
			{
				if (_input[i][j] != '#')
					continue;
				set.Add((i, j, 0, 0));
				amtCubes++;
			}

			var cyclesCompleted = 0;
			var min = -1;
			var max = _input.Length + 1;
			while (cyclesCompleted < 6)
			{
				var toAdd = new Set4Tuple();
				var toRemove = new Set4Tuple();
				for (var i = min; i < max; i++)
				for (var j = min; j < max; j++)
				for (var k = min; k < max; k++)
				for (var l = min; l < max; l++)
				{
					var neighbors = GetNeighbors(set, i, j, k, l);
					var activeNeighbors = neighbors.Count(x => x);

					if (set.Contains((i, j, k, l)) && activeNeighbors != 2 && activeNeighbors != 3)
					{
						toRemove.Add((i, j, k, l));
						amtCubes--;
						continue;
					}

					if (!set.Contains((i, j, k, l)) && activeNeighbors == 3)
					{
						toAdd.Add((i, j, k, l));
						amtCubes++;
					}
				}

				foreach (var add in toAdd)
					set.Add(add);

				foreach (var remove in toRemove)
					set.Remove(remove);

				min--;
				max++;
				cyclesCompleted++;
			}

			return amtCubes.ToString();
		}

		public static IList<bool> GetNeighbors(Set3Tuple set, int x, int y, int z)
		{
			var neighbors = new List<(int x, int y, int z)>();
			for (var a = -1; a <= 1; a++)
			for (var b = -1; b <= 1; b++)
			for (var c = -1; c <= 1; c++)
				if (a != 0 || b != 0 || c != 0)
					neighbors.Add((a, b, c));

			var neighborValues = new List<bool>();
			foreach (var (dx, dy, dz) in neighbors)
				neighborValues.Add(set.Contains((x + dx, y + dy, z + dz)));

			return neighborValues;
		}

		public static IList<bool> GetNeighbors(Set4Tuple set, int x, int y, int z, int w)
		{
			var neighbors = new List<(int x, int y, int z, int w)>();
			for (var a = -1; a <= 1; a++)
			for (var b = -1; b <= 1; b++)
			for (var c = -1; c <= 1; c++)
			for (var d = -1; d <= 1; d++)
				if (a != 0 || b != 0 || c != 0 || d != 0)
					neighbors.Add((a, b, c, d));

			var neighborValues = new List<bool>();
			foreach (var (dx, dy, dz, dw) in neighbors)
				neighborValues.Add(set.Contains((x + dx, y + dy, z + dz, w + dw)));

			return neighborValues;
		}
	}
}