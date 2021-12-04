using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/2
	/// </summary>
	public class Day02 : BaseDay
	{
		private readonly IList<string> _input;

		public Day02(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToList();

		public override string SolvePart1()
		{
			var validPasswords = 0;

			foreach (var line in _input)
			{
				var data = line.Split(" ");
				var min = int.Parse(data[0].Split("-")[0]);
				var max = int.Parse(data[0].Split("-")[1]);
				var letter = char.Parse(data[1].Replace(":", string.Empty));
				var password = data[2];

				var counter = password.Count(character => character == letter);

				if (min <= counter && counter <= max)
					validPasswords++;
			}

			return validPasswords.ToString();
		}

		public override string SolvePart2()
		{
			var validPasswords = 0;
			foreach (var line in _input)
			{
				var data = line.Split(" ");
				var leftIndex = int.Parse(data[0].Split("-")[0]);
				var rightIndex = int.Parse(data[0].Split("-")[1]);
				var letter = char.Parse(data[1].Replace(":", string.Empty));
				var password = data[2];

				if (password[leftIndex - 1] == letter ^ password[rightIndex - 1] == letter)
					validPasswords++;
			}

			return validPasswords.ToString();
		}
	}
}