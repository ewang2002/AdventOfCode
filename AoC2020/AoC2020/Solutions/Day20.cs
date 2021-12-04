using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/20
	/// </summary>
	public class Day20 : BaseDay
	{
		private readonly string[] _input;

		public Day20(string input)
			=> _input = input
				.Split(Environment.NewLine + Environment.NewLine)
				.ToArray();

		private readonly List<Tile> _allTiles = new();

		public override string SolvePart1()
		{
			// Step 1: create all tiles
			foreach (var tile in _input)
			{
				var keyVal = tile.Split(":")
					.Select(x => x.Trim())
					.ToArray();

				var key = int.Parse(keyVal[0].Split("Tile ")[1]);
				var value = keyVal[1]
					.Split(Environment.NewLine)
					.Select(x => x.ToCharArray())
					.ToArray();
				var matrix = new char[10, 10];
				for (var i = 0; i < value.Length; i++)
				for (var j = 0; j < value[i].Length; j++)
					matrix[i, j] = value[i][j];

				_allTiles.Add(new Tile(key, matrix));
			}

			// Step 2: check which tile has only 2 edges that meet
			// 2/4 edges means corner
			// check all outer edges first
			for (var i = 0; i < _allTiles.Count; i++)
			{
				var outerTile = _allTiles[i];
				// check all inner edges
				foreach (var innerTile in _allTiles)
				{
					if (outerTile.Id == innerTile.Id)
						continue;

					foreach (var outerEdge in outerTile.Edges)
					foreach (var innerEdge in innerTile.Edges)
					{
						if (!outerEdge.SequenceEqual(innerEdge) && !outerEdge.SequenceEqual(innerEdge.Reverse()))
							continue;

						_allTiles[i].MetEdges.Add(innerTile.Id);
					}
				}
			}

			// Step 3: get the answer
			var ans = 1L;
			foreach (var tile in _allTiles)
				// 2 means only 2 sides had a corresponding edge
				if (tile.MetEdges.Count == 2)
					ans *= tile.Id;

			return ans.ToString();
		}

		/// <summary>
		/// Constructs an image. Took a few thousand years for me to figure out. 
		/// </summary>
		/// <returns>The image.</returns>
		private Tile ConstructImage()
		{
			var lenOfSide = (int) Math.Sqrt(_allTiles.Count);
			var image = new Tile[lenOfSide, lenOfSide];

			// Establish the first corner tile. 
			var firstCornerTile = _allTiles
				.First(x => x.MetEdges.Count == 2);
			var usedTiles = new List<Tile> {firstCornerTile};

			var neighboringTiles = firstCornerTile.MetEdges
				// We dont want tiles that have already been used.
				.Where(x => !usedTiles.Select(a => a.Id).Contains(x))
				.Select(x => _allTiles.First(z => z.Id == x))
				.ToArray();

			// Basically orient the corner tile until the neighboring tiles "agree" 
			for (var i = 0; i < 8; i++)
			{
				InternalModifyTile(neighboringTiles[0], i);
				for (var j = 0; j < 8; j++)
				{
					InternalModifyTile(neighboringTiles[1], j);
					for (var k = 0; k < 8; k++)
					{
						InternalModifyTile(firstCornerTile, k);

						if (firstCornerTile.RightEdge.SequenceEqual(neighboringTiles[1].LeftEdge)
						    && firstCornerTile.BottomEdge.SequenceEqual(neighboringTiles[0].TopEdge))
							goto finishedCorner;

						if (firstCornerTile.RightEdge.SequenceEqual(neighboringTiles[0].LeftEdge)
						    && firstCornerTile.BottomEdge.SequenceEqual(neighboringTiles[1].TopEdge))
							goto finishedCorner;
					}
				}
			}

			finishedCorner:
			image[0, 0] = firstCornerTile;

			// Now we can go through each possible tile 
			for (var i = 0; i < lenOfSide; i++)
			{
				for (var j = 0; j < lenOfSide; j++)
				{
					if (i == 0 && j == 0)
						continue;

					// The left-most tiles
					// We need to check the top neighbors instead of the left tile. 
					if (i != 0 && j == 0)
					{
						var topTile = image[i - 1, 0];
						var possibleTiles = _allTiles
							.Where(x => x.MetEdges.Contains(topTile.Id) && !usedTiles.Contains(x))
							.ToArray();

						Tile foundRotatedTile = null;
						foreach (var t in possibleTiles)
						{
							if (t.TopEdge.SequenceEqual(topTile.BottomEdge))
							{
								foundRotatedTile = t;
								break;
							}

							foreach (var orientedTile in GetPossibleOrientations(t))
							{
								if (!orientedTile.TopEdge.SequenceEqual(topTile.BottomEdge))
									continue;

								foundRotatedTile = orientedTile;
								break;
							}

							if (foundRotatedTile != null)
								break;
						}

						if (foundRotatedTile == null)
							throw new Exception("Something went wrong when trying to find the right tile.");

						usedTiles.Add(foundRotatedTile);
						image[i, j] = foundRotatedTile;
						continue;
					}

					// Otherwise, check the left tile (like normal). 
					var prevTile = image[i, j - 1];
					var possTileToUse = _allTiles
						.Where(x => x.MetEdges.Contains(prevTile.Id) && !usedTiles.Contains(x))
						.ToArray();

					Tile rotatedTile = null;
					foreach (var t in possTileToUse)
					{
						if (t.LeftEdge.SequenceEqual(prevTile.RightEdge))
						{
							rotatedTile = t;
							break;
						}

						foreach (var orientedTile in GetPossibleOrientations(t))
						{
							if (!orientedTile.LeftEdge.SequenceEqual(prevTile.RightEdge))
								continue;

							rotatedTile = orientedTile;
							break;
						}

						if (rotatedTile != null)
							break;
					}

					if (rotatedTile == null)
						throw new Exception("Something went wrong when trying to find the right tile.");
					usedTiles.Add(rotatedTile);
					image[i, j] = rotatedTile;
				}
			}


			// Now we can combine the reconstructed image to make one big image
			// Convert each tile to a string for easier parsing 
			var strMatrix = new string[lenOfSide, lenOfSide];
			for (var x = 0; x < image.GetLength(0); x++)
			{
				for (var y = 0; y < image.GetLength(1); y++)
				{
					var sb = new StringBuilder();
					for (var z = 1; z < image[x, y].Arrangement.GetLength(0) - 1; z++)
					{
						for (var w = 1; w < image[x, y].Arrangement.GetLength(1) - 1; w++)
							sb.Append(image[x, y].Arrangement[z, w]);

						sb.AppendLine();
					}

					strMatrix[x, y] = sb.ToString();
				}
			}

			// Combine it all 
			var finalStrBuilder = new StringBuilder();
			for (var x = 0; x < strMatrix.GetLength(0); x++)
			{
				var z = 0;
				// It is assumed that z = 10 because the tiles are
				// 10 x 10 
				// Then we subtract 2 because we want to get rid of the borders
				while (z < 8)
				{
					for (var y = 0; y < strMatrix.GetLength(1); y++)
						finalStrBuilder.Append(strMatrix[x, y].Split(Environment.NewLine)[z]);

					finalStrBuilder.AppendLine();
					z++;
				}
			}

			// And now break it up. 
			var imgStr = finalStrBuilder.ToString();
			var imgStrToCharArray = imgStr
				.Split(Environment.NewLine)
				.Select(x => x.ToCharArray())
				.ToArray();
			var lenOfOneSide = imgStr.Split(Environment.NewLine)[0].Length;
			var newMatrix = new char[lenOfOneSide, lenOfOneSide];
			for (var i = 0; i < imgStrToCharArray.Length; i++)
			for (var j = 0; j < imgStrToCharArray[i].Length; j++)
				newMatrix[i, j] = imgStrToCharArray[i][j];
			return new Tile(1, newMatrix);
		}

		// There are 8 possible orientations a tile can be.
		// Go through each action four times
		// i.e. x % 2, where x <= 7. 
		private readonly Action<Tile>[] _allActions =
		{
			t => t.FlipTile(Tile.FlipAcross.HorizontalLine),
			t =>
			{
				t.FlipTile(Tile.FlipAcross.HorizontalLine);
				t.FlipTile(Tile.FlipAcross.VerticalLine);
			}
		};

		/// <summary>
		/// Internally modifies the tile's orientation. 
		/// </summary>
		/// <param name="t">The tile.</param>
		/// <param name="i">The index to access the list of actions.</param>
		private void InternalModifyTile(Tile t, int i)
		{
			if (i != 0 && i % 2 == 0)
			{
				t.FlipTile(Tile.FlipAcross.VerticalLine);
				t.RotateTile();
			}

			_allActions[i % 2](t);
		}

		/// <summary>
		/// Gets all possible orientations for this tile. 
		/// </summary>
		/// <param name="t">The tile.</param>
		/// <returns>The orientations that can be used.</returns>
		private IList<Tile> GetPossibleOrientations(Tile t)
		{
			var l = new List<Tile>();
			var clonedCopy = t.DeepClone();
			for (var i = 0; i < 8; i++)
			{
				InternalModifyTile(clonedCopy, i);
				l.Add(clonedCopy.DeepClone());
			}

			var anotherCopy = t.DeepClone();
			for (var i = 0; i < 4; i++)
			{
				anotherCopy.RotateTile();
				l.Add(anotherCopy.DeepClone());
			}

			return l;
		}

		public override string SolvePart2()
		{
			var image = ConstructImage();
			// number of '#'
			var numOfPound = 0;
			for (var i = 0; i < image.Arrangement.GetLength(0); i++)
			for (var j = 0; j < image.Arrangement.GetLength(1); j++)
				numOfPound += image[i, j] == '#' ? 1 : 0;
			
			var allPossibleOrientations = GetPossibleOrientations(image);
			var possNumOfSeaMonsters = -1;
			var numPoundForSeaMonster = -1; 
			foreach (var img in allPossibleOrientations)
			{
				var numSeaMonsters = 0;
				var used = new List<(int i, int j)>();
				for (var i = 0; i < img.Arrangement.GetLength(0); i++)
				{
					for (var j = 0; j < img.Arrangement.GetLength(1); j++)
					{
						var (isSeaMonster, coordsUsed) = IsSeaMonster(img, i, j, used);
						if (!isSeaMonster) 
							continue;
						
						numSeaMonsters++;
						used.AddRange(coordsUsed);
					}
				}

				if (possNumOfSeaMonsters < numSeaMonsters)
				{
					possNumOfSeaMonsters = numSeaMonsters;
					numPoundForSeaMonster = used.Count;
				}
			}

			return (numOfPound - numPoundForSeaMonster).ToString();
		}

		private static (bool isSeaMonster, List<(int i, int j)> coordsUsed)
			IsSeaMonster(Tile image, int i, int j, List<(int i, int j)> alreadyUsed)
		{
			var seaMonsterBody = new[]
			{
				(0, 18),
				(1, 0),
				(1, 5),
				(1, 6),
				(1, 11),
				(1, 12),
				(1, 17),
				(1, 18),
				(1, 19),
				(2, 1),
				(2, 4),
				(2, 7),
				(2, 10),
				(2, 13),
				(2, 16)
			};

			var used = new List<(int i, int j)>();
			foreach (var (di, dj) in seaMonsterBody)
			{
				if (i + di >= 0 && i + di < image.Arrangement.GetLength(0)
				                && j + dj >= 0 && j + dj < image.Arrangement.GetLength(1)
				                && image[i + di, j + dj] == '#'
				                && !alreadyUsed.Contains((i + di, j + dj)))
				{
					used.Add((i + di, j + dj));
					continue;
				}

				return (false, new List<(int i, int j)>());
			}

			return (true, used);
		}
	}

	public class Tile
	{
		/// <summary>
		/// How the tile looks.
		/// </summary>
		public char[,] Arrangement { get; private set; }

		/// <summary>
		/// The original tile arrangement.
		/// </summary>
		public char[,] OriginalArrangement { get; }

		/// <summary>
		/// The edges. [0] => Top; [1] => Right; [2] => Bottom; [3] => Left
		/// </summary>
		public IList<char>[] Edges { get; private set; }

		/// <summary>
		/// The top edge.
		/// </summary>
		public IList<char> TopEdge { get; private set; }

		/// <summary>
		/// The right edge.
		/// </summary>
		public IList<char> RightEdge { get; private set; }

		/// <summary>
		/// The bottom edge.
		/// </summary>
		public IList<char> BottomEdge { get; private set; }

		/// <summary>
		/// The left edge.
		/// </summary>
		public IList<char> LeftEdge { get; private set; }

		/// <summary>
		/// The tile's ID.
		/// </summary>
		public int Id { get; }

		/// <summary>
		/// All the "linked" tiles.
		/// </summary>
		public List<int> MetEdges { get; } = new();

		/// <summary>
		/// Creates a new Tile object. 
		/// </summary>
		/// <param name="id">The Tile's ID</param>
		/// <param name="arrangement">The arrangement.</param>
		public Tile(int id, char[,] arrangement)
		{
			Id = id;
			Arrangement = arrangement;
			OriginalArrangement = arrangement.Clone() as char[,];
			UpdateEdges();
		}

		/// <summary>
		/// Resets the tile back to its original version (when it was found). 
		/// </summary>
		public void ResetToOriginal()
		{
			Arrangement = OriginalArrangement.Clone() as char[,];
			UpdateEdges();
		}

		/// <summary>
		/// Returns a deep clone of this tile. 
		/// </summary>
		/// <returns>A deep clone of this tile.</returns>
		public Tile DeepClone()
			=> new(Id, Arrangement.Clone() as char[,]);

		private void UpdateEdges()
		{
			// get tile info
			var topEdge = new List<char>();
			var bottomEdge = new List<char>();
			var leftEdge = new List<char>();
			var rightEdge = new List<char>();
			for (var i = 0; i < Arrangement.GetLength(0); i++)
			{
				topEdge.Add(Arrangement[0, i]);
				bottomEdge.Add(Arrangement[Arrangement.GetLength(0) - 1, i]);
				leftEdge.Add(Arrangement[i, 0]);
				rightEdge.Add(Arrangement[i, Arrangement.GetLength(1) - 1]);
			}

			Edges = new IList<char>[]
			{
				topEdge,
				rightEdge,
				bottomEdge,
				leftEdge
			};

			TopEdge = topEdge;
			BottomEdge = bottomEdge;
			LeftEdge = leftEdge;
			RightEdge = rightEdge;
		}

		/// <summary>
		/// Flips the tile across the horizontal or vertical line.
		/// </summary>
		/// <param name="flipAcross">The direction to flip the tile to.</param>
		public void FlipTile(FlipAcross flipAcross)
		{
			var newMatrix = new char[Arrangement.GetLength(0), Arrangement.GetLength(1)];
			switch (flipAcross)
			{
				case FlipAcross.HorizontalLine:
					for (var i = 0; i < Arrangement.GetLength(0) / 2; i++)
					for (var j = 0; j < Arrangement.GetLength(1); j++)
					{
						newMatrix[Arrangement.GetLength(0) - 1 - i, j] = Arrangement[i, j];
						newMatrix[i, j] = Arrangement[Arrangement.GetLength(0) - 1 - i, j];
					}

					break;
				case FlipAcross.VerticalLine:
					for (var i = 0; i < Arrangement.GetLength(0); i++)
					for (var j = 0; j < Arrangement.GetLength(1) / 2; j++)
					{
						newMatrix[i, Arrangement.GetLength(1) - 1 - j] = Arrangement[i, j];
						newMatrix[i, j] = Arrangement[i, Arrangement.GetLength(1) - 1 - j];
					}

					break;
				default:
					throw new ArgumentOutOfRangeException(nameof(flipAcross), flipAcross, null);
			}

			Arrangement = newMatrix;
			UpdateEdges();
		}

		/// <summary>
		/// Rotates the tile 90 degrees clockwise. 
		/// </summary>
		public void RotateTile(int amt = 1)
		{
			amt %= 4;
			while (amt != 0)
			{
				var newMatrix = new char[Arrangement.GetLength(0), Arrangement.GetLength(1)];
				for (var r = 0; r < Arrangement.GetLength(0); r++)
				for (var c = 0; c < Arrangement.GetLength(1); c++)
					newMatrix[c, Arrangement.GetLength(0) - 1 - r] = Arrangement[r, c];

				Arrangement = newMatrix;
				amt--;
			}

			UpdateEdges();
		}

		/// <summary>
		/// Prints the tiles. 
		/// </summary>
		public void PrintTile()
			=> Console.WriteLine(ToString());

		/// <summary>
		/// Returns the string representation of this tile.
		/// </summary>
		/// <returns>The string representation of the tile object.</returns>
		public override string ToString()
		{
			var sb = new StringBuilder()
				.Append($"ID: {Id}")
				.AppendLine();
			for (var i = 0; i < Arrangement.GetLength(0); i++)
			{
				for (var j = 0; j < Arrangement.GetLength(1); j++)
					sb.Append(Arrangement[i, j]);
				sb.AppendLine();
			}

			return sb.ToString();
		}

		public enum FlipAcross
		{
			HorizontalLine,
			VerticalLine
		}

		public char this[int i, int j] => Arrangement[i, j];
	}
}