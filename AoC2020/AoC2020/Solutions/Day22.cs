using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC2020.Solutions
{
	/// <summary>
	/// https://adventofcode.com/2020/day/22
	/// </summary>
	public class Day22 : BaseDay
	{
		private readonly string[] _input;

		public Day22(string input)
			=> _input = input
				.Split(Environment.NewLine + Environment.NewLine)
				.ToArray();

		private readonly List<int> _player1Cards = new();
		private readonly List<int> _player2Cards = new();

		public override string SolvePart1()
		{
			var player1Cards = _input[0].Split(Environment.NewLine)
				.Skip(1)
				.Select(int.Parse)
				.ToList();
			var player2Cards = _input[1].Split(Environment.NewLine)
				.Skip(1)
				.Select(int.Parse)
				.ToList();

			_player1Cards.AddRange(player1Cards);
			_player2Cards.AddRange(player2Cards);

			while (player1Cards.Count != 0 && player2Cards.Count != 0)
			{
				var p1Card = player1Cards[0];
				var p2Card = player2Cards[0];
				player1Cards.RemoveAt(0);
				player2Cards.RemoveAt(0);

				if (p1Card > p2Card)
				{
					player1Cards.Add(p1Card);
					player1Cards.Add(p2Card);
					continue;
				}

				player2Cards.Add(p2Card);
				player2Cards.Add(p1Card);
			}

			var finalScore = 0;
			if (player1Cards.Count == 0)
			{
				for (var i = 0; i < player2Cards.Count; i++)
					finalScore += player2Cards[i] * (player2Cards.Count - i);
			}
			else
			{
				for (var i = 0; i < player1Cards.Count; i++)
					finalScore += player1Cards[i] * (player1Cards.Count - i);
			}

			return finalScore.ToString();
		}

		public override string SolvePart2()
		{
			var player1Cards = _player1Cards.ToList();
			var player2Cards = _player2Cards.ToList();
			
			while (player1Cards.Count != 0 && player2Cards.Count != 0)
			{
				var p1Card = player1Cards[0];
				var p2Card = player2Cards[0];
				player1Cards.RemoveAt(0);
				player2Cards.RemoveAt(0);

				if (p1Card <= player1Cards.Count && p2Card <= player2Cards.Count)
				{
					var res = PlaySubGame(player1Cards.GetRange(0, p1Card), player2Cards.GetRange(0, p2Card));
					if (res)
					{
						player1Cards.Add(p1Card);
						player1Cards.Add(p2Card);
					}
					else
					{
						player2Cards.Add(p2Card);
						player2Cards.Add(p1Card);
					}
				}
				else
				{
					if (p1Card > p2Card)
					{
						player1Cards.Add(p1Card);
						player1Cards.Add(p2Card);
					}
					else
					{
						player2Cards.Add(p2Card);
						player2Cards.Add(p1Card);
					}
				}
			}
			
			var finalScore = 0;
			if (player1Cards.Count == 0)
			{
				for (var i = 0; i < player2Cards.Count; i++)
					finalScore += player2Cards[i] * (player2Cards.Count - i);
			}
			else
			{
				for (var i = 0; i < player1Cards.Count; i++)
					finalScore += player1Cards[i] * (player1Cards.Count - i);
			}
			
			return finalScore.ToString();
		}

		private bool PlaySubGame(IList<int> p1Cards, IList<int> p2Cards)
		{
			var player1Cards = p1Cards.ToList();
			var player2Cards = p2Cards.ToList();

			var previousP1Cards = new List<IList<int>>();
			var previousP2Cards = new List<IList<int>>();

			while (player1Cards.Count != 0 && player2Cards.Count != 0)
			{

				var isFound = false;
				foreach (var prev in previousP1Cards)
					if (player1Cards.SequenceEqual(prev))
					{
						isFound = true;
						break;
					}

				foreach (var prev in previousP2Cards)
					if (player2Cards.SequenceEqual(prev))
					{
						isFound = true;
						break;
					}

				if (isFound)
					return true;

				previousP1Cards.Add(player1Cards.ToList());
				previousP2Cards.Add(player2Cards.ToList());
				
				var p1Card = player1Cards[0];
				var p2Card = player2Cards[0];
				player1Cards.RemoveAt(0);
				player2Cards.RemoveAt(0);

				if (p1Card <= player1Cards.Count && p2Card <= player2Cards.Count)
				{
					var res = PlaySubGame(player1Cards.GetRange(0, p1Card), player2Cards.GetRange(0, p2Card));
					if (res)
					{
						player1Cards.Add(p1Card);
						player1Cards.Add(p2Card);
					}
					else
					{
						player2Cards.Add(p2Card);
						player2Cards.Add(p1Card);
					}
				}
				else
				{
					if (p1Card > p2Card)
					{
						player1Cards.Add(p1Card);
						player1Cards.Add(p2Card);
					}
					else
					{
						player2Cards.Add(p2Card);
						player2Cards.Add(p1Card);
					}
				}
			}

			return player1Cards.Count != 0;
		}
	}
}