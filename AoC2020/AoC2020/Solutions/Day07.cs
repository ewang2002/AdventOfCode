using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/7
	/// </summary>
	public class Day07 : BaseDay
	{
		private readonly IList<string> _input;

		public Day07(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToList();

		public override string SolvePart1()
		{
			var rules = new Dictionary<string, string[]>();
			// parse input
			foreach (var line in _input)
			{
				var keyValPair = line.Split("contain");
				var key = keyValPair[0].Trim()[..^1];
				if (keyValPair[1].Contains("no other bag"))
				{
					rules.Add(key, Array.Empty<string>());
					continue;
				}

				var val = keyValPair[1].Trim().Split(", ")
					// get bag name only
					.Select(x => x[2..])
					.Select(x => x.EndsWith('s') ? x[..^1] : x)
					// replace plural bag at end
					.Select(x => x.Replace("s.", string.Empty))
					// replace singular bag at end
					.Select(x => x.Replace(".", string.Empty))
					.ToArray();
				rules.Add(key, val);
			}

			// check which ones has a gold bag
			var bagsThatHasGold = new HashSet<string>();
			foreach (var (bag, items) in rules)
				if (items.Contains("shiny gold bag"))
					bagsThatHasGold.Add(bag);

			var oldLength = bagsThatHasGold.Count;
			while (true)
			{
				foreach (var (bag, items) in rules)
				{
					var ct = items.Count(x => bagsThatHasGold.Contains(x));
					if (ct == 0)
						continue;
					bagsThatHasGold.Add(bag);
				}

				if (oldLength == bagsThatHasGold.Count)
					break;

				oldLength = bagsThatHasGold.Count;
			}

			return bagsThatHasGold.Count.ToString();
		}

		private IDictionary<string, (int, string)[]> _rulesPt2;

		public override string SolvePart2()
		{
			_rulesPt2 = new Dictionary<string, (int, string)[]>();
			// parse input
			foreach (var line in _input)
			{
				var keyValPair = line.Split("contain");
				var key = keyValPair[0].Trim()[..^1];
				if (keyValPair[1].Contains("no other bag"))
				{
					_rulesPt2.Add(key, Array.Empty<(int, string)>());
					continue;
				}

				var val = keyValPair[1].Trim().Split(", ");

				var valList = new List<(int qty, string bag)>();
				foreach (var b in val)
				{
					if (b.Contains("no other bag"))
						break;

					var amt = int.Parse(b[..2].Trim());
					var bag = b[2..];
					if (bag.EndsWith('s'))
						bag = bag[..^1];
					bag = bag.Replace("s.", string.Empty)
						.Replace(".", string.Empty);
					valList.Add((amt, bag));
				}

				_rulesPt2.Add(key, valList.ToArray());
			}

			return Calculate(_rulesPt2["shiny gold bag"]).ToString();
		}

		private int Calculate(IReadOnlyCollection<(int, string)> bags)
		{
			if (bags.Count == 0)
				return 0;

			var val = bags.Sum(x => x.Item1);
			foreach (var (qty, bag) in bags)
				val += qty * Calculate(_rulesPt2[bag]);

			return val;
		}
	}
}