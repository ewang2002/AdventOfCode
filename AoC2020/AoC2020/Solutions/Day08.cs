using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/8
	/// </summary>
	public class Day08 : BaseDay
	{
		private readonly IList<string> _input;

		public Day08(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToList();

		public override string SolvePart1()
		{
			var instructions = _input
				.Select(x => x.Split(" "))
				.ToArray();

			var acc = 0;
			
			var visitedLine = new HashSet<int>();
			var index = 0;
			while (true)
			{
				if (visitedLine.Contains(index))
					break;

				visitedLine.Add(index);
				if (instructions[index][0] == "nop")
				{
					index++;
					continue;
				}

				var num = int.Parse(instructions[index][1].Replace("+", string.Empty));
				if (instructions[index][0] == "acc")
				{
					acc += num;
					index++; 
					continue;
				}

				if (instructions[index][0] == "jmp")
					index += num; 
			}

			return acc.ToString();
		}

		public override string SolvePart2()
		{
			var acc = 0;
			var prevIndex = 0;

			while (true)
			{
				var success = false;
				// i hate references 
				// why do references suck 
				var clonedInput = _input
					.Select(x => x.Split(" "))
					.ToList();

				for (; prevIndex < clonedInput.Count; prevIndex++)
				{
					if (clonedInput[prevIndex][0] == "nop")
					{
						clonedInput[prevIndex][0] = "jmp";
						prevIndex++;
						break;
					}
					
					if (clonedInput[prevIndex][0] == "jmp")
					{
						clonedInput[prevIndex][0] = "nop";
						prevIndex++;
						break;
					}
				}

				var index = 0;
				var visitedLine = new HashSet<int>();

				while (true)
				{
					if (index >= clonedInput.Count)
					{
						success = true;
						break;
					}

					if (visitedLine.Contains(index))
						break;

					visitedLine.Add(index);
					if (clonedInput[index][0] == "nop")
					{
						index++;
						continue;
					}

					var num = int.Parse(clonedInput[index][1].Replace("+", string.Empty));
					if (clonedInput[index][0] == "acc")
					{
						acc += num;
						index++;
						continue;
					}

					if (clonedInput[index][0] == "jmp")
						index += num;
				}

				if (success)
					break;

				acc = 0;
			}

			return acc.ToString();
		}
	}
}