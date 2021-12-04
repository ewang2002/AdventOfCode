using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/15
	/// </summary>
	public class Day15 : BaseDay
	{
		private readonly int[] _input;

		public Day15(string input)
			=> _input = input
				.Split(',')
				.Select(int.Parse)
				.ToArray();

		public override string SolvePart1()
			=> GetSpokenNumber(at: 2020).ToString();

		public override string SolvePart2()
			=> GetSpokenNumber(at: 30000000).ToString();

		public int GetSpokenNumber(int at)
		{
			var numsSaid = new int[at];
			Array.Fill(numsSaid, -1);
			var i = 0;
			for (; i < _input.Length - 1; i++)
				numsSaid[_input[i]] = i;

			var lastNum = _input[^1];
			for (; i < at - 1; i++)
			{
				// last number spoken once only
				if (numsSaid[lastNum] == -1)
				{
					numsSaid[lastNum] = i;
					lastNum = 0;
					continue;
				}

				var idx = numsSaid[lastNum];
				numsSaid[lastNum] = i;
				lastNum = i - idx;
			}

			return lastNum;
		}

		/*
		public int GetSpokenNumber(int at)
		{
			var numsSaid = new Dictionary<int, (int f, int l)>();
			// index = turns + 1
			var i = 0;
			for (; i < _input.Length; i++)
				numsSaid.Add(_input[i], (f: -1, l: i));

			var lastNum = _input.Last();
			for (; i < at; i++)
			{
				if (numsSaid.ContainsKey(lastNum) && numsSaid[lastNum].f != -1)
				{
					var newNum = i - 1 - numsSaid[lastNum].f;
					lastNum = newNum;
					if (numsSaid.ContainsKey(newNum))
						numsSaid[newNum] = (f: numsSaid[newNum].l, l: i);
					else
						numsSaid.Add(newNum, (f: -1, l: i));

					continue;
				}

				lastNum = 0;
				if (numsSaid.ContainsKey(0))
					numsSaid[0] = (f: numsSaid[0].l, l: i);
				else
					numsSaid.Add(0, (f: -1, l: i));
			}

			return lastNum;
		}*/

		/*
		public int GetSpokenNumber(int at)
		{
			var numsSaid = new Dictionary<int, IList<int>>();
			// index = turns + 1
			var i = 0;
			for (; i < _input.Length; i++)
				numsSaid.Add(_input[i], new List<int> { i });

			var lastNum = _input.Last();
			for (; i < at; i++)
			{
				if (numsSaid.ContainsKey(lastNum) && numsSaid[lastNum].Count > 1)
				{
					var newNum = numsSaid[lastNum][^1] - numsSaid[lastNum][^2];
					lastNum = newNum;
					if (numsSaid.ContainsKey(newNum))
					{
						numsSaid[newNum].Add(i);
						if (numsSaid[newNum].Count > 2)
							numsSaid[newNum].RemoveAt(0);
					}
					else
						numsSaid.Add(newNum, new List<int> { i });
					continue;
				}

				lastNum = 0;
				if (numsSaid.ContainsKey(0))
					numsSaid[0].Add(i);
				else
					numsSaid.Add(0, new List<int> { i });
			}

			return lastNum;
		}*/

		/*
		public override string SolvePart1()
		{
			var numsSaid = new List<int>(2020);
			// index = turns + 1
			foreach (var num in _input)
				numsSaid.Add(num);

			while (numsSaid.Count != 2020)
			{
				var subset = numsSaid.GetRange(0, numsSaid.Count - 2);
				if (subset.Contains(numsSaid[^1]))
				{
					var lastTwoTurns = new int[2];
					var amt = 0;
					for (var i = numsSaid.Count - 1; i >= 0 && amt < 2; i--)
						if (numsSaid[i] == numsSaid[^1])
							lastTwoTurns[amt++] = i;

					numsSaid.Add(lastTwoTurns[0] - lastTwoTurns[1]);
					continue;
				}

				numsSaid.Add(0);
			}

			return numsSaid[^1].ToString();
		}*/
	}
}