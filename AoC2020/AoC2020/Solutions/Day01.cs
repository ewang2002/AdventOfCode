using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/1
	/// </summary>
	public class Day01 : BaseDay
	{
		private readonly ISet<int> _nums;

		public Day01(string input)
			=> _nums = input
				.Split(Environment.NewLine)
				.Select(int.Parse)
				.ToHashSet();

		public override string SolvePart1()
		{
			foreach (var i in _nums)
			foreach (var j in _nums)
				if (i + j == 2020)
					return (i * j).ToString();

			return "-1";
		}

		public override string SolvePart2()
		{
			foreach (var i in _nums)
			foreach (var j in _nums)
			foreach (var k in _nums)
				if (i + j + k == 2020)
					return (i * j * k).ToString();

			return "-1";
		}
	}
}