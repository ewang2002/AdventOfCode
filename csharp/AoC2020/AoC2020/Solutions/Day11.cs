using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/11
	/// </summary>
	public class Day11 : BaseDay
	{
		private readonly List<char[]> _input;
		private const char EmptySeat = 'L';
		private const char Floor = '.';
		private const char OccupiedSeat = '#';

		public Day11(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.Select(x => x.ToCharArray())
				.Distinct()
				.ToList();

		public override string SolvePart1()
		{
			var original = -1;
			var inputArr = DeepClone(_input);

			while (true)
			{
				var clonedArray = DeepClone(inputArr);

				var change = 0;
				for (var i = 0; i < _input.Count; i++)
				for (var j = 0; j < _input.Count; j++)
				{
					if (CanOccupy(inputArr, i, j, true))
					{
						clonedArray[i][j] = OccupiedSeat;
						change++;
					}

					if (CanEmpty(inputArr, i, j, true, 4))
					{
						clonedArray[i][j] = EmptySeat;
						change--;
					}
				}

				if (change == original)
					break;

				original = change;
				inputArr = DeepClone(clonedArray);
			}

			return inputArr.SelectMany(charArr => charArr)
				.Count(c => c == '#')
				.ToString();
		}


		public override string SolvePart2()
		{
			var original = -1;
			var inputArr = DeepClone(_input);

			while (true)
			{
				var clonedArray = DeepClone(inputArr);

				var change = 0;
				for (var i = 0; i < _input.Count; i++)
				for (var j = 0; j < _input.Count; j++)
				{
					if (CanOccupy(inputArr, i, j, false))
					{
						clonedArray[i][j] = OccupiedSeat;
						change++;
					}

					if (CanEmpty(inputArr, i, j, false, 5))
					{
						clonedArray[i][j] = EmptySeat;
						change--;
					}
				}

				if (change == original)
					break;

				original = change;
				inputArr = DeepClone(clonedArray);
			}

			return inputArr.SelectMany(charArr => charArr)
				.Count(c => c == '#')
				.ToString();
		}

		private static bool CanEmpty(IList<char[]> arr, int i, int j, bool checkAdj, int tol)
		{
			if (arr[i][j] == Floor || arr[i][j] == EmptySeat)
				return false;

			return checkAdj
				? GetAdjacentElements(arr, i, j).Count(x => x == OccupiedSeat) >= tol
				: GetFirstElementInEachDir(arr, i, j).Count(x => x == OccupiedSeat) >= tol;
		}

		private static bool CanOccupy(IList<char[]> arr, int i, int j, bool checkAdj)
		{
			if (arr[i][j] == OccupiedSeat || arr[i][j] == Floor)
				return false;

			return checkAdj 
				? GetAdjacentElements(arr, i, j).All(x => x != OccupiedSeat)
				: GetFirstElementInEachDir(arr, i, j).All(x => x != OccupiedSeat);
		}

		private static IEnumerable<char> GetFirstElementInEachDir(IList<char[]> arr, int i, int j)
		{
			var result = new List<char>();

			// check curr to bottom
			for (var a = i; a < arr.Count; a++)
				if ((arr[a][j] == OccupiedSeat || arr[a][j] == EmptySeat) && a != i)
				{
					result.Add(arr[a][j]);
					break;
				}

			// check curr to top
			for (var a = i; a >= 0; a--)
				if ((arr[a][j] == OccupiedSeat || arr[a][j] == EmptySeat) && a != i)
				{
					result.Add(arr[a][j]);
					break;
				}

			// check curr to right
			for (var b = j; b < arr[i].Length; b++)
				if ((arr[i][b] == OccupiedSeat || arr[i][b] == EmptySeat) && b != j)
				{
					result.Add(arr[i][b]);
					break;
				}

			// check curr to left
			for (var b = j; b >= 0; b--)
				if ((arr[i][b] == OccupiedSeat || arr[i][b] == EmptySeat) && b != j)
				{
					result.Add(arr[i][b]);
					break;
				}

			var c = i;
			var d = j;
			// top left
			while (c - 1 >= 0 && d - 1 >= 0)
			{
				c--;
				d--;

				if (arr[c][d] != OccupiedSeat && arr[c][d] != EmptySeat)
					continue;

				result.Add(arr[c][d]);
				break;
			}

			// top right
			c = i;
			d = j;
			while (c - 1 >= 0 && d + 1 < arr.Count)
			{
				c--;
				d++;

				if (arr[c][d] != OccupiedSeat && arr[c][d] != EmptySeat)
					continue;

				result.Add(arr[c][d]);
				break;
			}

			// bottom left
			c = i;
			d = j;
			while (c + 1 < arr.Count && d - 1 >= 0)
			{
				c++;
				d--;

				if (arr[c][d] != OccupiedSeat && arr[c][d] != EmptySeat)
					continue;

				result.Add(arr[c][d]);
				break;
			}

			// bottom right
			c = i;
			d = j;
			while (c + 1 < arr.Count && d + 1 < arr.Count)
			{
				c++;
				d++;

				if (arr[c][d] != OccupiedSeat && arr[c][d] != EmptySeat)
					continue;

				result.Add(arr[c][d]);
				break;
			}

			return result;
		}

		private static IEnumerable<char> GetAdjacentElements(IList<char[]> arr, int i, int j)
		{
			var adjacentDiff = new (int x, int y)[]
			{
				(-1, 0), // top
				(1, 0), // bottom
				(0, 1), // right
				(0, -1), // left
				(-1, -1), // top left
				(1, -1), // top right
				(-1, 1), // bottom left
				(1, 1) // bottom right
			};

			var res = new List<char>();
			foreach (var (x, y) in adjacentDiff)
			{
				var newI = i + x;
				var newJ = j + y;
				if (newI < 0 || newI >= arr.Count)
					continue;

				if (newJ < 0 || newJ >= arr[newI].Length)
					continue;

				res.Add(arr[newI][newJ]);
			}

			return res;
		}

		private static IList<char[]> DeepClone(IList<char[]> arr)
		{
			var newArr = new List<char[]>();
			for (var i = 0; i < arr.Count; i++)
			{
				newArr.Add(new char[arr[i].Length]);
				for (var j = 0; j < arr.Count; j++)
					newArr[i][j] = arr[i][j];
			}

			return newArr;
		}
	}
}