using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/10
	/// </summary>
	public class Day10 : BaseDay
	{
		private readonly List<int> _input;

		public Day10(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(int.Parse)
				.OrderBy(x => x)
				.Distinct()
				.ToList();

		public override string SolvePart1()
		{
			// input.Add(_input.Max() + 3);
			var one = 0;
			var three = 1;

			for (var i = 1; i < _input.Count; i++)
			{
				switch (_input[i] - _input[i - 1])
				{
					case 1:
						one++;
						break;
					case 3:
						three++;
						break;
				}
			}

			return (one * three).ToString();
		}

		public override string SolvePart2()
			=> GetCombination(0, 0).ToString();

		private readonly IDictionary<int, ulong> _dict = new Dictionary<int, ulong>();

		// We're going to use "memorization" to "cache" each values in a dictionary. 
		// This will significantly speed things up. 
		public ulong GetCombination(int index, int effectiveRating)
		{
			if (_dict.ContainsKey(effectiveRating))
				return _dict[effectiveRating];

			var eRating = effectiveRating;
			var combos = (ulong)0;

			for (var i = index; i < _input.Count; i++)
			{
				var elementsLeft = _input.Count - i;
				var subset = _input.GetRange(i, elementsLeft >= 3 ? 3 : elementsLeft)
					.Select(x => x - 3 <= eRating ? x : -1)
					.ToArray();

				var numOfNegOne = subset.Count(x => x != -1);

				if (numOfNegOne > 1)
				{
					for (var j = 0; j < subset.Length; j++)
					{
						if (subset[j] == -1)
							continue;

						// get number of possible combinations that led up to the end of the array
						var c = GetCombination(i + j + 1, subset[j]);
						_dict.TryAdd(subset[j], c);
						combos += c;
					}

					break;
				}

				if (numOfNegOne == 0)
					break;

				eRating = subset.First(x => x != -1);
			}

			// TODO how does this work? 
			return eRating == _input.Max() 
				? 1 // hit the end of the "tree" 
				: combos;
		}

		/*
				public ulong GetCombination(int index, int effectiveRating)
				{
					var combos = (ulong) 0;

					for (var i = index; i < _input.Count; i++)
					{
						var elementsLeft = _input.Count - i;
						var subset = _input.GetRange(i, elementsLeft >= 3 ? 3 : elementsLeft)
							.Select(x => x - 3 <= effectiveRating ? x : -1)
							.ToArray();

						var nunmOfNegOne = subset.Count(x => x != -1);

						if (nunmOfNegOne > 1)
						{
							for (var j = 0; j < subset.Length; j++)
							{
								if (subset[j] == -1)
									continue;

								combos++; 
								combos += GetCombination(i + j + 1, subset[j]);
							}

							break;
						}

						if (nunmOfNegOne == 0)
							break;

						effectiveRating = subset.First(x => x != -1);
					}

					return combos;
				}*/
	}
}