using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	using Set2Tuple = HashSet<(int x, int y)>;

	/// <summary>
	/// https://adventofcode.com/2020/day/24
	/// </summary>
	public class Day24 : BaseDay
	{
		private readonly string[] _input;

		public Day24(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToArray();


		public override string SolvePart1()
		{
			var tiles = new Set2Tuple();

			foreach (var line in _input)
			{
				var directionsToTake = new List<string>();
				var c = 0;
				while (c < line.Length)
				{
					var first = line[c];
					if (first == 'e' || first == 'w')
					{
						directionsToTake.Add(first.ToString());
						c++;
						continue;
					}

					directionsToTake.Add(line[c] + line[c + 1].ToString());
					c += 2;
				}

				var x = 0;
				var y = 0;
				foreach (var d in directionsToTake)
				{
					switch (d)
					{
						// X Y
						// e n ++
						// w s --
						case "e":
							x += 2;
							break;
						case "se":
							x++;
							y--;
							break;
						case "sw":
							x--;
							y--;
							break;
						case "w":
							x -= 2;
							break;
						case "nw":
							x--;
							y++;
							break;
						case "ne":
							x++;
							y++;
							break;
					}
				}

				if (tiles.Contains((x, y)))
					tiles.Remove((x, y));
				else
					tiles.Add((x, y));
			}

			_blackTiles = tiles;
			return tiles.Count.ToString();
		}

		private Set2Tuple _blackTiles;

		public override string SolvePart2()
		{
			var blackTiles = _blackTiles;

			var cyclesCompleted = 0;
			var min = Math.Min(blackTiles.Min(x => x.x), blackTiles.Min(x => x.y)) - 3;
			var max = Math.Max(blackTiles.Max(x => x.x), blackTiles.Max(x => x.y)) + 3;
			while (cyclesCompleted < 100)
			{
				var toRemove = new Set2Tuple();
				var toAdd = new Set2Tuple();
				cyclesCompleted++;
				for (var x = min; x < max; x++)
				for (var y = min; y < max; y++)
				{
					var neighbors = GetNeighbors(blackTiles, x, y);
					var numBlack = neighbors.Where(x => x).ToArray();
					if (blackTiles.Contains((x, y)) && (numBlack.Length == 0 || numBlack.Length > 2))
					{
						toRemove.Add((x, y));
						continue;
					}

					if (!blackTiles.Contains((x, y)) && numBlack.Length == 2)
						toAdd.Add((x, y));
				}

				foreach (var r in toRemove)
					blackTiles.Remove(r);

				foreach (var a in toAdd)
					blackTiles.Add(a);

				min -= 3;
				max += 3;
			}

			return blackTiles.Count.ToString();
		}

		public static IList<bool> GetNeighbors(Set2Tuple set, int x, int y)
		{
			var neighbors = new List<(int x, int y)>
			{
				(2, 0),
				(1, -1),
				(-1, -1),
				(-2, 0),
				(-1, 1),
				(1, 1)
			};

			var neighborValues = new List<bool>();
			foreach (var (dx, dy) in neighbors)
				neighborValues.Add(set.Contains((x + dx, y + dy)));

			return neighborValues;
		}
	}
}