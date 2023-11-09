using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

// Not going to lie. This is a terrible solution. 
// You shouldn't look ahead. Your eyes won't be able to handle it.
// Instead, look at the revised solution. :) 
namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/17
	/// </summary>
	public class Day17 : BaseDay
	{
		private readonly char[][] _input;

		public Day17(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(x => x.ToCharArray())
				.ToArray();

		public override string SolvePart1()
		{
			// # = active
			// . = inactive

			var activeCubes = 0;
			// probably don't need to allocate this many elements 
			var cube = new bool[_input.Length + 25, _input.Length + 25, _input.Length + 25];
			var mid = cube.GetLength(0) / 2 - _input[0].Length / 2;
			for (var i = 0; i < _input.Length; i++)
			for (var j = 0; j < _input[i].Length; j++)
			{
				cube[i + mid, j + mid, mid] = _input[i][j] == '#';
				activeCubes += _input[i][j] == '#' ? 1 : 0;
			}

			var cyclesCompleted = 0;
			while (cyclesCompleted < 6)
			{
				Debug.Assert(cube != null, nameof(cube) + " != null");
				var cloneCube = cube.Clone() as bool[,,];
				for (var i = 0; i < cube.GetLength(0); i++)
				for (var j = 0; j < cube.GetLength(1); j++)
				for (var k = 0; k < cube.GetLength(2); k++)
				{
					var neighborsAtPt = GetNeighbors(cube, i, j, k);
					// -1 because we counted this position as well. 
					var numTrue = neighborsAtPt.Count(x => x);

					if (cube[i, j, k] && numTrue != 3 && numTrue != 4)
					{
						cloneCube![i, j, k] = false;
						activeCubes--;
						continue;
					}

					if (!cube[i, j, k] && numTrue == 3)
					{
						cloneCube![i, j, k] = true;
						activeCubes++;
					}
				}

				cube = cloneCube;
				cyclesCompleted++;
			}

			return activeCubes.ToString();
		}


		public override string SolvePart2()
		{
			// # = active
			// . = inactive

			var activeCubes = 0;
			// probably don't need to allocate this many elements 
			var cube = new bool[_input.Length + 25, _input.Length + 25, _input.Length + 25, _input.Length + 25];
			var mid = cube.GetLength(0) / 2 - _input[0].Length / 2;
			for (var i = 0; i < _input.Length; i++)
			for (var j = 0; j < _input[i].Length; j++)
			{
				cube[i + mid, j + mid, mid, mid] = _input[i][j] == '#';
				activeCubes += _input[i][j] == '#' ? 1 : 0;
			}

			var cyclesCompleted = 0;
			while (cyclesCompleted < 6)
			{
				Debug.Assert(cube != null, nameof(cube) + " != null");
				var cloneCube = cube.Clone() as bool[,,,];
				for (var i = 0; i < cube.GetLength(0); i++)
				for (var j = 0; j < cube.GetLength(1); j++)
				for (var k = 0; k < cube.GetLength(2); k++)
				for (var l = 0; l < cube.GetLength(3); l++)
				{
					var neighborsAtPt = GetNeighbors(cube, i, j, k, l);
					// -1 because we counted this position as well. 
					var numTrue = neighborsAtPt.Count(x => x);

					if (cube[i, j, k, l] && numTrue != 3 && numTrue != 4)
					{
						cloneCube![i, j, k, l] = false;
						activeCubes--;
						continue;
					}

					if (!cube[i, j, k, l] && numTrue == 3)
					{
						cloneCube![i, j, k, l] = true;
						activeCubes++;
					}
				}

				cube = cloneCube;
				cyclesCompleted++;
			}

			return activeCubes.ToString();
		}

		private static IList<bool> GetNeighbors(bool[,,] arr, int i, int j, int k)
		{
			var neighbors = new List<(int x, int y, int z)>();

			for (var a = -1; a <= 1; a++)
			for (var b = -1; b <= 1; b++)
			for (var c = -1; c <= 1; c++)
			{
				if (a + i >= 0 && a + i < arr.GetLength(0)
				               && b + j >= 0 && b + j < arr.GetLength(1)
				               && c + k >= 0 && c + k < arr.GetLength(2))
					neighbors.Add((a, b, c));
			}

			var neighborsAtCoords = new List<bool>();
			foreach (var (x, y, z) in neighbors)
				neighborsAtCoords.Add(arr[i + x, j + y, k + z]);

			return neighborsAtCoords;
		}

		private static IList<bool> GetNeighbors(bool[,,,] arr, int i, int j, int k, int l)
		{
			var neighbors = new List<(int x, int y, int z, int w)>();

			for (var a = -1; a <= 1; a++)
			for (var b = -1; b <= 1; b++)
			for (var c = -1; c <= 1; c++)
			for (var d = -1; d <= 1; d++)
			{
				if (a + i >= 0 && a + i < arr.GetLength(0)
				               && b + j >= 0 && b + j < arr.GetLength(1)
				               && c + k >= 0 && c + k < arr.GetLength(2)
				               && d + l >= 0 && d + l < arr.GetLength(3))
					neighbors.Add((a, b, c, d));
			}

			var neighborsAtCoords = new List<bool>();
			foreach (var (x, y, z, w) in neighbors)
				neighborsAtCoords.Add(arr[i + x, j + y, k + z, l + w]);

			return neighborsAtCoords;
		}
	}
}