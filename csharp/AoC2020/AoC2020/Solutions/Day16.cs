using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/16
	/// </summary>
	public class Day16 : BaseDay
	{
		private readonly string[] _input;

		public Day16(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToArray();

		public void ParseInput()
		{
			_range = new Dictionary<string, IList<(int l, int r)>>();
			var i = 0;
			for (; i < _input.Length; i++)
			{
				if (string.IsNullOrEmpty(_input[i]))
					break;

				var keyVals = _input[i]
					.Split(": ");
				var values = keyVals[1]
					.Split(" or ");
				_range.Add(keyVals[0], new List<(int l, int r)>());
				foreach (var value in values)
				{
					var nums = value.Split("-")
						.Select(int.Parse)
						.ToArray();
					_range[keyVals[0]].Add((l: nums[0], r: nums[1]));
				}
			}

			i += 2;
			_myTicket = _input[i].Split(',')
				.Select(int.Parse)
				.ToArray();
			i += 3;
			_nearbyTickets = new List<int[]>();
			for (; i < _input.Length; i++)
			{
				var numsInLine = _input[i].Split(',')
					.Select(int.Parse)
					.ToArray();
				_nearbyTickets.Add(numsInLine);
			}
		}

		private Dictionary<string, IList<(int l, int r)>> _range;
		private int[] _myTicket;
		private List<int[]> _nearbyTickets;

		public override string SolvePart1()
		{
			ParseInput();
			var invalidTickets = new List<int>();
			foreach (var ticket in _nearbyTickets.SelectMany(x => x).ToArray())
			{
				var isOk = false;
				foreach (var (_, checker) in _range)
				{
					foreach (var (l, r) in checker)
					{
						if (l > ticket || ticket > r) 
							continue;
						isOk = true;
						goto getOut;
					}
				}

				getOut:
				if (isOk)
					continue;

				invalidTickets.Add(ticket);
				for (var a = 0; a < _nearbyTickets.Count; a++)
					for (var b = 0; b < _nearbyTickets[a].Length; b++)
						if (_nearbyTickets[a][b] == ticket)
							_nearbyTickets[a][b] = -1;
			}

			return invalidTickets.Sum().ToString();
		}

		public override string SolvePart2()
		{
			var dict = new Dictionary<string, IList<int>>();
			
			// step 1: tranpose the list
			var tranposedList = _nearbyTickets
				.SelectMany(x => x.Select((item, index) => new { item, index }))
				.GroupBy(i => i.index, i => i.item)
				.Select(g => g.ToList())
				.ToList();
			
			// step 2: see where each list of nums will fit into the constraint 
			for (var i = 0; i < tranposedList.Count; i++)
			{
				foreach (var (key, val) in _range)
				{
					bool IsValidNumber(int num) => val[0].l <= num && num <= val[0].r
					                               || val[1].l <= num && num <= val[1].r;

					var amtPassed = tranposedList[i]
						.Where(x => x != -1)
						.Count(x => !IsValidNumber(x));

					if (amtPassed != 0) 
						continue;

					if (dict.ContainsKey(key))
						dict[key].Add(i);
					else
						dict[key] = new List<int> {i};
				}
			}

			Debug.Assert(dict.Count == _nearbyTickets[0].Length);
			
			// step 3: "unwrap" the dictionary
			var dictWithOneVal = new Dictionary<string, int>();
			while (true)
			{
				var changed = false;
				foreach (var (key, val) in dict)
				{
					if (val.Count != 1)
						continue;
					
					dictWithOneVal.Add(key, val[0]);
					dict.Remove(key);
					changed = true;
					foreach (var k in dict.Keys.Where(k => k != key))
						dict[k].Remove(val[0]);
				}
				
				if (!changed)
					break;
			}

			var ans = 1L;
			foreach (var k in dictWithOneVal.Keys.Where(x => x.StartsWith("departure")))
				ans *= _myTicket[dictWithOneVal[k]];
			
			return ans.ToString();
		}
	}
}