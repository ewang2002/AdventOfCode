using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/5
	/// </summary>
	public class Day05 : BaseDay
	{
		private readonly IList<string> _input;

		public Day05(string input)
			=> _input = input
				.Split(Environment.NewLine)
				.ToList();

		private IList<int> _ids; 

		public override string SolvePart1()
		{
			var maxId = 0;
			var allSeatIds = new List<int>();
			foreach (var boardingPass in _input)
			{
				var rows = boardingPass[..7];
				var cols = boardingPass[7..];

				// begin with rows
				var minRow = 0;
				var maxRow = 127;
				for (var i = 0; i < rows.Length - 1; i++)
				{
					if (rows[i] == 'F')
						maxRow = (int) Math.Floor(maxRow - (maxRow - minRow) / 2.0);
					else
						minRow = (int) Math.Ceiling(minRow + (maxRow - minRow) / 2.0);
				}

				var rowVal = rows[^1] == 'F'
					? minRow
					: maxRow;

				// cols
				var minCol = 0;
				var maxCol = 7;
				for (var i = 0; i < cols.Length - 1; i++)
				{
					if (cols[i] == 'L')
						maxCol = (int) Math.Floor(maxCol - (maxCol - minCol) / 2.0);
					else
						minCol = (int) Math.Ceiling(minCol + (maxCol - minCol) / 2.0);
				}

				var colVal = cols[^1] == 'L'
					? minCol
					: maxCol;

				var seatId = rowVal * 8 + colVal;
				allSeatIds.Add(seatId);
				if (seatId > maxId)
					maxId = seatId;
			}

			_ids = allSeatIds; 
			return maxId.ToString();
		}

		public override string SolvePart2()
		{
			Debug.Assert(_ids != null, "_ids != null");
			_ids = _ids
				.OrderBy(x => x)
				.ToList();
			for (var i = 1; i < _ids.Count; i++)
			{
				if (_ids[i] - _ids[i - 1] == 2)
					return (_ids[i] - 1).ToString();
			}

			return string.Empty;
		}
	}
}