#define EFFICIENT
using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.RevisedSolutions
{
	public class Day09Revised : BaseDay
	{
		private readonly List<long> _input;

		public Day09Revised(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(long.Parse)
				.ToList();

		private readonly int _preamble = 25;
		private long _ans;

		public override string SolvePart1()
		{
			for (var i = 0; i < _input.Count; i++)
			{
				var subset = _input.GetRange(i, _preamble);
				var targetNumber = _input[i + _preamble];

#if EFFICIENT
				var isFound = false;
				foreach (var num in subset)
				foreach (var num2 in subset)
					if (num != num2 && num + num2 == targetNumber)
					{
						isFound = true;
						goto outLoop;
					}

				outLoop:
				if (isFound)
					continue;
#else
				var len = (from n1 in subset
					from n2 in subset
					where n1 != n2 && n1 + n2 == targetNumber
					select (n1, n2)).Count();

				if (len != 0)
					continue;
#endif

				  _ans = targetNumber;
				return targetNumber.ToString();
			}

			return (-1).ToString();
		}

		public override string SolvePart2()
		{
			var ct = 2;
			while (true)
			{
				for (var i = ct; i < _input.Count; i++)
				{
					var subset = _input.GetRange(i - ct, ct);
					if (subset.Sum() == _ans)
						return (subset.Min() + subset.Max()).ToString();

					if (subset.Sum() > _ans)
						break;
				}

				ct++;
			}
		}
	}
}