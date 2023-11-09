using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/6
	/// </summary>
	public class Day06 : BaseDay
	{
		private readonly IList<string> _input;

		public Day06(string input)
			=> _input = input
				.Split(Environment.NewLine + Environment.NewLine)
				.ToList();

		public override string SolvePart1()
		{
			var groupAnsweredYes = new List<int>();
			foreach (var question in _input)
				groupAnsweredYes.Add(string.Join("", question.Split(Environment.NewLine))
					.ToCharArray()
					.Distinct()
					.Count());

			return groupAnsweredYes.Sum().ToString();
		}

		public override string SolvePart2()
		{
			var sharedYes = 0;
			foreach (var question in _input)
			{
				var dict = new Dictionary<char, int>();
				var allResponsesInGroup = question.Split(Environment.NewLine);
				foreach (var response in allResponsesInGroup)
				{
					var charArr = response.ToCharArray()
						.Distinct();
					foreach (var c in charArr)
						if (dict.ContainsKey(c))
							dict[c]++;
						else
							dict.Add(c, 1);
				}

				foreach (var (_, e) in dict)
					if (e == allResponsesInGroup.Length)
						sharedYes++; 
			}

			return sharedYes.ToString();
		}
	}
}