using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/3
	/// </summary>
	public class Day03 : BaseDay
	{
		private readonly IList<string> _input;

		public Day03(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToList();

		public override string SolvePart1()
		{
			// i -> down
			// j -> right
			var i = 0;
			var j = 0;
			// . = free
			// # = tree
			var numTrees = 0;

			while (true)
			{
				i += 1;
				j += 3;
				if (i >= _input.Count)
					break;

				while (j >= _input[i].Length)
					_input[i] += _input[i];

				if (_input[i][j] == '#')
					numTrees++;
			}

			return numTrees.ToString();
		}

		public override string SolvePart2()
		{
			var totalTreeCount = new List<int>();
			var directions = new List<int[]>
			{
				new[] {1, 1},
				new[] {3, 1},
				new[] {5, 1},
				new[] {7, 1},
				new[] {1, 2}
			};

			foreach (var direction in directions)
			{
				var i = 0;
				var j = 0;
				var numTrees = 0;
				while (true)
				{
					i += direction[1];
					j += direction[0];
					if (i >= _input.Count)
						break;

					while (j >= _input[i].Length)
						_input[i] += _input[i];

					if (_input[i][j] == '#')
						numTrees++;
				}

				totalTreeCount.Add(numTrees);
			}

			return totalTreeCount
				.Aggregate(1, (current, treeCount) => current * treeCount)
				.ToString();
		}
	}
}