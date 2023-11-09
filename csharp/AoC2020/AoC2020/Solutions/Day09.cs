using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/9
	/// </summary>
	public class Day09 : BaseDay
	{
		private readonly List<long> _input;

		public Day09(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(long.Parse)
				.ToList();

		private readonly int _preamble = 25;

		public override string SolvePart1()
		{
			var prevIndex = 0;

			while (true)
			{
				var numsToUse = new HashSet<long>();
				for (var i = 0; i < _preamble; i++)
					numsToUse.Add(_input[i + prevIndex]);

				var isFound = false;
				foreach (var num in numsToUse)
				{
					foreach (var num2 in numsToUse)
					{
						if (num == num2)
							continue;

						if (num + num2 != _input[_preamble + prevIndex]) 
							continue;

						isFound = true;
						break;
					}

					if (isFound)
						break;
				}

				if (!isFound)
					return _input[_preamble + prevIndex].ToString();

				prevIndex++;
			}
		}

		public override string SolvePart2()
		{
			var val = long.Parse(SolvePart1());
			var contiguous = 2;

			while (true)
			{
				for (var i = contiguous; i < _input.Count; i++)
				{

					var newArr = _input
						.GetRange(i - contiguous, contiguous)
						.ToArray();

					// Alt way of doing it
					// _input.Skip(i - contiguous)
					//		.Take(contiguous)
					//		.ToList(); 

					if (newArr.Sum() != val) 
						continue;

					newArr = newArr.OrderBy(x => x).ToArray();
					return (newArr[0] + newArr[^1]).ToString();
				}

				contiguous++; 
			}
		}
	}
}