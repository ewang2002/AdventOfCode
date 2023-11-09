using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Linq;

namespace AoC2020.RevisedSolutions
{
	public class Day12Revised : BaseDay
	{
		private readonly List<(char dir, int dist)> _input;

		public Day12Revised(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(x => (x[0], int.Parse(x[1..])))
				.ToList();

		private readonly char[] _possAngles = {'E', 'S', 'W', 'N'};

		public override string SolvePart1()
		{
			var x = 0;
			var y = 0;
			// 0 = east
			// 90 = south
			// 180 = west
			// 270 = north
			var angle = 0;

			foreach (var (dir, val) in _input)
			{
				switch (dir)
				{
					case 'L':
						ChangeDirection(ref angle, -val);
						continue;
					case 'R':
						ChangeDirection(ref angle, val);
						continue;
				}

				var direction = dir == 'F'
					? _possAngles[angle / 90]
					: dir;

				switch (direction)
				{
					case 'N':
						y += val;
						break;
					case 'S':
						y -= val;
						break;
					case 'E':
						x += val;
						break;
					case 'W':
						x -= val;
						break;
				}
			}

			return (Math.Abs(x) + Math.Abs(y)).ToString();
		}

		public static void ChangeDirection(ref int currAngle, int angle)
		{
			angle %= 360;
			if (angle < 0) angle = 360 - Math.Abs(angle);
			currAngle = (currAngle + angle) % 360;
		}

		[SuppressMessage("Microsoft.Style", "IDE0042")]
		public override string SolvePart2()
		{
			(int x, int y) ship = (0, 0);
			(int x, int y) waypoint = (10, 1);

			foreach (var (dir, val) in _input)
			{
				switch (dir)
				{
					case 'L':
						ChangeWaypointDirection(ref waypoint, -val);
						continue;
					case 'R':
						ChangeWaypointDirection(ref waypoint, val);
						continue;
					case 'F':
						ship.x += waypoint.x * val;
						ship.y += waypoint.y * val;
						continue;
					case 'N':
						waypoint.y += val;
						break;
					case 'S':
						waypoint.y -= val;
						break;
					case 'E':
						waypoint.x += val;
						break;
					case 'W':
						waypoint.x -= val;
						break;
				}
			}

			return (Math.Abs(ship.x) + Math.Abs(ship.y)).ToString();
		}

		public static void ChangeWaypointDirection(ref (int x, int y) waypoint, int dir)
		{
			dir %= 360;
			if (dir < 0)
				dir = 360 - Math.Abs(dir);

			var x = waypoint.x;
			switch (dir)
			{
				case 90:
					waypoint.x = waypoint.y;
					waypoint.y = -x;
					break;
				case 180:
					waypoint.x = -waypoint.x;
					waypoint.y = -waypoint.y;
					break;
				case 270:
					waypoint.x = -waypoint.y;
					waypoint.y = x;
					break;
			}
		} 
	}
}