using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/13
	/// </summary>
	public class Day13 : BaseDay
	{
		private readonly IList<string> _input;

		public Day13(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToList();

		public override string SolvePart1()
		{
			var targetTime = int.Parse(_input[0]);
			var possibleBuses = _input[1].Split(',')
				.Where(x => x != "x")
				.Select(int.Parse)
				.ToArray();

			// K = id
			// V = time closet to targetTime
			var dict = new Dictionary<int, int>();
			foreach (var busId in possibleBuses)
			{
				var time = 0;
				while (time < targetTime)
					time += busId;

				dict.Add(busId, time);
			}

			dict = dict.OrderBy(x => x.Value)
				.ToDictionary(x => x.Key, x => x.Value);


			return (dict.First().Key * (dict.First().Value - targetTime)).ToString();
		}

		// wtf is this
		public override string SolvePart2()
		{
			var possibleBuses = _input[1].Split(',');
			var busTimeOffset = new List<(int id, int offset)>();
			for (var i = 0; i < possibleBuses.Length; i++)
			{
				if (possibleBuses[i] == "x")
					continue;

				busTimeOffset.Add((int.Parse(possibleBuses[i]), i));
			}

			var time = (long) busTimeOffset[0].id;
			var step = (long) busTimeOffset[0].id;

			for (var i = 1; i < busTimeOffset.Count; i++)
			{
				while ((time + busTimeOffset[i].offset) % busTimeOffset[i].id != 0)
					time += step;
				step *= busTimeOffset[i].id;
			}

			return time.ToString();
		}

		/*
		public override string SolvePart2()
		{
			var possibleBuses = _input[1].Split(',')
				.ToArray();

			var delayBuses = new Dictionary<int, int>();
			var allBuses = new Dictionary<int, long>();
			var delay = 1;
			foreach (var bus in possibleBuses)
			{
				delay++;
				if (bus == "x")
					continue;

				allBuses.Add(int.Parse(bus), int.Parse(bus) * 1000000000000);
				delayBuses.Add(int.Parse(bus), delay - 1);
				delay = 1;
			}

			while (true)
			{
				var keys = allBuses.Keys.ToArray();
				allBuses[keys[^1]] += keys[^1];

				for (var i = keys.Length - 2; i >= 0; i--)
					while (allBuses[keys[i]] + keys[i] < allBuses[keys[i + 1]])
						allBuses[keys[i]] += keys[i];

				if (IsLinedUp(delayBuses, allBuses))
					return allBuses.Values.Min().ToString();
			}
		}

		public bool IsLinedUp(Dictionary<int, int> delayBuses, Dictionary<int, long> allBuses)
		{
			var time = -1L;
			var passed = true;
			foreach (var (id, delayTime) in delayBuses)
			{
				if (time == -1)
				{
					time = allBuses[id];
					Console.WriteLine(time);
					continue;
				}

				if (allBuses[id] - delayTime != time)
				{
					passed = false;
					break;
				}

				time = allBuses[id];
			}

			return passed;
		}*/
	}
}