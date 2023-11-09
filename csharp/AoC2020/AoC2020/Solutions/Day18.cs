using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/18
	/// </summary>
	public class Day18 : BaseDay
	{
		private readonly string[] _input;

		public Day18(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToArray();

		public override string SolvePart1()
			=> EvaluateInput(true).ToString();

		public override string SolvePart2()
			=> EvaluateInput(false).ToString();


		private long EvaluateInput(bool p1)
		{
			var sum = 0L;
			foreach (var equation in _input)
			{
				// so I can edit 
				var equ = equation.Replace(" ", string.Empty);
				while (equ.Contains('(') && equ.Contains(')'))
				{
					var s = -1;
					var e = -1;
					for (var i = equ.IndexOf('('); i < equ.LastIndexOf(')') + 1; i++)
					{
						if (equ[i] == '(')
						{
							s = i;
							continue;
						}

						if (equ[i] == ')')
						{
							e = i;
							break;
						}
					}

					var res = EvaluateExpression(equ[s..(e + 1)], p1);
					equ = equ.Replace(equ[s..(e + 1)], res.ToString());
				}

				sum += EvaluateExpression(equ, p1);
			}

			return sum;
		}

		private static long EvaluateExpression(string exp, bool p1)
		{
			exp = exp.Replace("(", string.Empty)
				.Replace(")", string.Empty);

			if (p1)
			{
				var numbers = exp.Split('*', '+')
					.Select(long.Parse)
					.ToArray();
				var operators = exp.ToCharArray()
					.Where(x => !char.IsNumber(x))
					.ToArray();
				var sum = numbers[0];

				Debug.Assert(numbers.Length - 1 == operators.Length);
				for (var i = 1; i < numbers.Length; i++)
				{
					switch (operators[i - 1])
					{
						case '+':
							sum += numbers[i];
							break;
						case '*':
							sum *= numbers[i];
							break;
					}
				}

				return sum;
			}

			// for part 2 
			var unparsedNumbers = exp.Split('*')
				.ToArray();

			var l = new List<long>();
			foreach (var expression in unparsedNumbers)
			{
				var allNumbersInExp = expression.Split('+')
					.Select(long.Parse)
					.ToArray();
				l.Add(allNumbersInExp.Sum());
			}

			return l.Aggregate(1L, (current, n) => current * n);
		}
	}
}