using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/19
	/// </summary>
	public class Day19 : BaseDay
	{
		private readonly string[] _input;

		public Day19(string input)
			=> _input = input
				.Split(Environment.NewLine + Environment.NewLine)
				.ToArray();

		private void ParseInput()
		{
			var unparsedRules = _input[0]
				.Split(Environment.NewLine)
				.Select(x => x.Split(": "))
				.ToArray();

			_ruleToLetter = new Dictionary<int, string>();
			_rules = new Dictionary<int, int[][]>();
			foreach (var rule in unparsedRules)
			{
				var ruleNum = int.Parse(rule[0]);
				var ruleInfo = rule[1];

				if (ruleInfo.StartsWith("\""))
				{
					_ruleToLetter.Add(ruleNum, ruleInfo[1..^1]);
					continue;
				}

				var allPossRules = ruleInfo
					.Split(" | ")
					.Select(x => x.Split(" ")
						.Select(int.Parse)
						.ToArray())
					.ToArray();
				_rules.Add(ruleNum, allPossRules);
			}

			_messages = _input[1]
				.Split(Environment.NewLine)
				.ToArray();
		}

		private Dictionary<int, int[][]> _rules;
		private Dictionary<int, string> _ruleToLetter;
		private string[] _messages;
		private readonly Dictionary<int, IList<string>> _cached = new();

		private bool IsValidMessage(string message)
		{
			var str = new StringBuilder();
			foreach (var num in _rules[0])
			{
				foreach (var n in num)
				{
					if (_ruleToLetter.ContainsKey(n))
					{
						str.Append(_ruleToLetter[n]);
						continue;
					}

					var combo = PossibleCombos(n);
					var isFound = false;
					foreach (var c in combo)
					{
						if (message.StartsWith(str + c))
						{
							isFound = true;
							str.Append(c);
							break;
						}
					}

					if (!isFound)
						return false;
				}
			}

			return str.ToString() == message;
		}

		private IList<string> PossibleCombos(int id)
		{
			if (_ruleToLetter.ContainsKey(id))
				return new List<string> {_ruleToLetter[id]};

			if (_cached.ContainsKey(id))
				return _cached[id];
			
			var l = new List<string>();
			foreach (var numbers in _rules[id])
			{
				var tempList = new List<IList<string>>();
				foreach (var number in numbers)
				{
					if (number == id)
						continue;

					var eval = PossibleCombos(number);
					if (!_cached.ContainsKey(number))
						_cached.Add(number, eval);
					
					tempList.Add(eval);
				}

				l.AddRange(GetAllPossibleCombosFromMultList(tempList));
				l = l.Distinct().ToList();
			}

			return l;
		}

		private static IList<string> GetAllPossibleCombosFromMultList(IEnumerable<IList<string>> strings)
		{
			IEnumerable<string> combos = new[] {string.Empty};

			foreach (var inner in strings)
				combos = from c in combos
					from i in inner
					select c + i;

			return combos.ToList();
		}

		public override string SolvePart1()
		{
			ParseInput();
			return _messages.Count(IsValidMessage).ToString();
		}

		public override string SolvePart2()
		{
			// some cached values contain old definitions 
			_cached.Clear();
			_rules[8] = new[]
			{
				new[] {42},
				new[] {42, 42},
				new[] {42, 42, 42},
				new[] {42, 42, 42, 42},
				new[] {42, 42, 42, 42, 42},
				new[] {42, 42, 42, 42, 42, 42},
				new[] {42, 42, 42, 42, 42, 42, 42},
				new[] {42, 42, 42, 42, 42, 42, 42, 42},
				new[] {42, 42, 42, 42, 42, 42, 42, 42, 42},
				new[] {42, 42, 42, 42, 42, 42, 42, 42, 42, 42},
			};

			_rules[11] = new[]
			{
				new[] {42, 31},
				new[] {42, 42, 31, 31},
				new[] {42, 42, 42, 31, 31, 31},
				new[] {42, 42, 42, 42, 31, 31, 31, 31},
				new[] {42, 42, 42, 42, 42, 31, 31, 31, 31, 31},
				new[] {42, 42, 42, 42, 42, 42, 31, 31, 31, 31, 31, 31},
				new[] {42, 42, 42, 42, 42, 42, 42, 31, 31, 31, 31, 31, 31, 31},
				new[] {42, 42, 42, 42, 42, 42, 42, 42, 31, 31, 31, 31, 31, 31, 31, 31},
				new[] {42, 42, 42, 42, 42, 42, 42, 42, 42, 31, 31, 31, 31, 31, 31, 31, 31, 31},
				new[] {42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31}
			};

			return _messages.Count(IsValidMessage).ToString();
		}
	}
}