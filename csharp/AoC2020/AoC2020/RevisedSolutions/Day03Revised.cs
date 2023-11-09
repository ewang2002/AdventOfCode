using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.RevisedSolutions
{
	public class Day03Revised : BaseDay
	{
		private readonly IList<string> _input;

		public Day03Revised(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToList();

		public int ComputeTrees(int right, int down)
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
				i += down;
				j += right;
				if (i >= _input.Count)
					break;

				while (j >= _input[i].Length)
					_input[i] += _input[i];

				if (_input[i][j] == '#')
					numTrees++;
			}

			return numTrees;
		}

		public override string SolvePart1()
			=> ComputeTrees(3, 1).ToString();

		public override string SolvePart2()
			=> new List<int[]>
				{
					new[] {1, 1},
					new[] {3, 1},
					new[] {5, 1},
					new[] {7, 1},
					new[] {1, 2}
				}
				.Select(direction => ComputeTrees(direction[0], direction[1]))
				.Aggregate(1, (current, treeCount) => current * treeCount)
				.ToString();
	}
}