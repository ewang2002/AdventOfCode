using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/12
	/// </summary>
	public class Day12 : BaseDay
	{
		private readonly List<(char dir, int dist)> _input;

		public Day12(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(x => (x[0], int.Parse(x[1..])))
				.ToList();

		public override string SolvePart1()
		{
			var currX = 0;
			var currY = 0;
			var currDir = 'E';

			foreach (var (dir, dist) in _input)
			{
				var direction = dir;
				switch (direction)
				{
					case 'F':
						direction = currDir;
						break;
					case 'L':
						ChangeDirection(ref currDir, dist, -1);
						continue;
					case 'R':
						ChangeDirection(ref currDir, dist, 1);
						continue;
				}

				switch (direction)
				{
					case 'N':
						currY += dist;
						break;
					case 'S':
						currY -= dist;
						break;
					case 'E':
						currX += dist;
						break;
					case 'W':
						currX -= dist;
						break;
				}
			}

			return (Math.Abs(currX) + Math.Abs(currY)).ToString();
		}

		private readonly char[] _angleOrder = {'E', 'S', 'W', 'N'};

		public void ChangeDirection(ref char currDirection, int angle, int dir)
		{
			angle = Math.Abs(angle) % 360 / 90;

			var index = -1;
			for (var i = 0; i < _angleOrder.Length; i++)
				if (_angleOrder[i] == currDirection)
				{
					index = i;
					break;
				}

			if (index == -1)
				return;

			if (dir == 1)
			{
				currDirection = _angleOrder[(index + angle) % 4];
				return;
			}

			for (var i = 0; i < angle; i++)
				index--; 

			currDirection = _angleOrder[index < 0 ? ^Math.Abs(index) : index];
		}

		[SuppressMessage("Microsoft.Style", "IDE0042")]
		public override string SolvePart2()
		{
			(int x, int y) ship = (0, 0);
			(int x, int y) wayPoint = (10, 1);

			foreach (var (dir, dist) in _input)
			{
#if PRINT
				Console.WriteLine("============");
				Console.WriteLine($"{dir}{dist} => Waypoint: {wayPoint}");			
#endif
				var direction = dir;
				switch (direction)
				{
					case 'F':
						ship.x += wayPoint.x * dist;
						ship.y += wayPoint.y * dist;
						continue;
					case 'L':
						ChangeWaypointDir(ref wayPoint, dist, -1);
#if PRINT
						Console.WriteLine($"\tNew Waypoint: {wayPoint}");
#endif
						continue;
					case 'R':
						ChangeWaypointDir(ref wayPoint, dist, 1);
#if PRINT
						Console.WriteLine($"\tNew Waypoint: {wayPoint}");
#endif
						continue;
				}

				switch (direction)
				{
					case 'N':
						wayPoint.y += dist;
						break;
					case 'S':
						wayPoint.y -= dist;
						break;
					case 'E':
						wayPoint.x += dist;
						break;
					case 'W':
						wayPoint.x -= dist;
						break;
				}
#if PRINT
				Console.WriteLine($"\tNew Waypoint: {wayPoint}");
				Console.WriteLine($"\tNew Ship: {ship}");		
#endif
			}

			return (Math.Abs(ship.x) + Math.Abs(ship.y)).ToString();
		}

		public static void ChangeWaypointDir(ref (int x, int y) waypoint, int angle, int dir)
		{
			angle = Math.Abs(angle) % 360 / 90;
			if (dir == -1)
				angle = 4 - angle;

			var x = waypoint.x;
			switch (angle)
			{
				case 1: // 90
					waypoint.x = waypoint.y;
					waypoint.y = -x;
					break;
				case 2: // 180
					waypoint.x = -waypoint.x;
					waypoint.y = -waypoint.y;
					break;
				case 3: // 270
					waypoint.x = -waypoint.y;
					waypoint.y = x;
					break;
			}
		}
	}
}