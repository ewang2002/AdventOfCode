using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/14
	/// </summary>
	public class Day14 : BaseDay
	{
		private readonly string[] _input;

		public Day14(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToArray();

		public override string SolvePart1()
		{
			// key = memory address
			// val = result 
			var memoryDict = new Dictionary<int, char[]>();
			var mask = _input[0].ToCharArray();
			foreach (var line in _input)
			{
				var lineVal = line.Split(" = ");
				Debug.Assert(lineVal.Length == 2);

				if (lineVal[0].StartsWith("mask"))
				{
					mask = lineVal[1].ToCharArray()
						.ToArray();
					continue;
				}

				var address = int.Parse(lineVal[0][4..^1]);
				var value = Convert.ToString(int.Parse(lineVal[1]), 2)
					.ToCharArray()
					.ToList();
				while (value.Count < 36)
					value.Insert(0, '0');

				Debug.Assert(value.Count == mask.Length);
				var clone = mask.Clone() as char[];
				for (var i = 0; i < clone!.Length; i++)
				{
					if (clone[i] != 'X')
						continue;

					clone[i] = value[i];
				}

				if (memoryDict.ContainsKey(address))
					memoryDict[address] = clone;
				else
					memoryDict.Add(address, clone);
			}

			var sum = memoryDict.Values
				.Sum(x => Convert.ToInt64(string.Join("", x), 2));

			return sum.ToString();
		}

		public override string SolvePart2()
		{
			var memoryDict = new Dictionary<long, long>();
			var mask = _input[0].ToCharArray();
			foreach (var line in _input)
			{
				var lineVal = line.Split(" = ");
				Debug.Assert(lineVal.Length == 2);

				if (lineVal[0].StartsWith("mask"))
				{
					mask = lineVal[1].ToCharArray()
						.ToArray();
					continue;
				}

				var address = int.Parse(lineVal[0][4..^1]);
				var addressBase2 = Convert.ToString(address, 2)
					.ToCharArray()
					.ToList();
				var val = int.Parse(lineVal[1]);
				while (addressBase2.Count < 36)
					addressBase2.Insert(0, '0');

				Debug.Assert(addressBase2.Count == mask.Length);

				var numX = 0;
				for (var i = 0; i < mask.Length; i++)
				{
					switch (mask[i])
					{
						case '0':
							continue;
						case '1':
							addressBase2[i] = '1';
							continue;
						case 'X':
							addressBase2[i] = 'X';
							numX++;
							break;
					}
				}

				var combos = GetBinaryCombination(numX);
				foreach (var combo in combos)
				{
					var numIndex = 0; 
					var clonedAddress = addressBase2.ToArray();
					for (var i = 0; i < clonedAddress.Length; i++)
					{
						if (clonedAddress[i] != 'X')
							continue;

						clonedAddress[i] = combo[numIndex++];
					}

					var dec = Convert.ToInt64(string.Join("", clonedAddress), 2);
					if (memoryDict.ContainsKey(dec))
						memoryDict[dec] = val; 
					else
						memoryDict.Add(dec, val);
				}
			}

			var sum = memoryDict.Values
				.Sum();

			return sum.ToString();
		}

		// implementation from stack overflow
		private static IList<char[]> GetBinaryCombination(int length)
		{
			var l = new List<char[]>();
			for (var i = 0L; i < 1 << length; i++)
			{
				l.Add(GetBinary(i, length).ToCharArray());
			}

			return l;
		}

		private static string GetBinary(long n, int l)
		{
			var bit = (long) (1 << l - 1);
			var str = new StringBuilder();
			while (bit != 0)
			{
				str.Append((n & bit) != 0 ? 1 : 0);
				bit >>= 1;
			}

			return str.ToString();
		}
	}
}