using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/23
	/// </summary>
	public class Day23 : BaseDay
	{
		private readonly int[] _input;

		public Day23(string input)
			=> _input = input
				.Select(x => int.Parse(x.ToString()))
				.ToArray();

		public override string SolvePart1()
		{
			var cups = _input.ToList();
			for (var i = 0; i < 100; i++)
			{
				var indexOfFirstCup = i % cups.Count;
				var firstCup = cups[indexOfFirstCup];

				var nextThreeCups = new List<int>();
				for (var j = 0; j < 3; j++)
					nextThreeCups.Add(cups[(i + 1 + j) % cups.Count]);

				foreach (var cup in nextThreeCups)
					cups.Remove(cup);

				var destinationCup = firstCup - 1;
				while (true)
				{
					if (cups.Contains(destinationCup) && !nextThreeCups.Contains(destinationCup))
						break;

					destinationCup--;
					if (destinationCup < cups.Min())
						destinationCup = cups.Max();
				}

				var idxOfDestination = cups.FindIndex(x => x == destinationCup);
				for (var a = nextThreeCups.Count - 1; a >= 0; a--)
					cups.Insert(idxOfDestination + 1, nextThreeCups[a]);

				while (cups[indexOfFirstCup] != firstCup)
				{
					var oldFirst = cups[0];
					cups.RemoveAt(0);
					cups.Add(oldFirst);
				}
			}

			while (cups[0] != 1)
			{
				var oldFirst = cups[0];
				cups.RemoveAt(0);
				cups.Add(oldFirst);
			}
			
			return string.Join("", cups).Replace("1", string.Empty);
		}

		public override string SolvePart2()
		{
			// Going to do something similar to what I did in day 15
			var arr = new int[1_000_000];
			for (var i = 0; i < _input.Length; i++)
				arr[_input[i]] = i;
			
			return string.Empty;
		}
	}
}