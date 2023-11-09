using System;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/25
	/// </summary>
	public class Day25 : BaseDay
	{
		private readonly long[] _input;

		public Day25(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(long.Parse)
				.ToArray();


		public override string SolvePart1()
		{
			var cardPublicKey = _input[0];
			var doorPublicKey = _input[1];
			
			var cardSubjectNumber = 1L;
			var cardLoopSize = 0;
			while (cardSubjectNumber != cardPublicKey)
			{
				cardSubjectNumber = 7 * cardSubjectNumber % 20201227;
				cardLoopSize++; 
			}

			var ans = 1L;
			for (var i = 0; i < cardLoopSize; i++)
				ans = doorPublicKey * ans % 20201227;

			return ans.ToString();
		}

		public override string SolvePart2()
		{
			// Apparently I need to finish the other puzzles first
			// Maybe some other time... 
			return string.Empty;
		}
	}
}