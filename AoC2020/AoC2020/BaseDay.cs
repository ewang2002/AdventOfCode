using System;
using System.Diagnostics;

namespace AoC2020
{
	public abstract class BaseDay
	{
		/// <summary>
		/// Solution to part 1.
		/// </summary>
		/// <returns>The solution.</returns>
		public abstract string SolvePart1();

		/// <summary>
		/// Solution to part 2.
		/// </summary>
		/// <returns>The solution.</returns>
		public abstract string SolvePart2();

		/// <summary>
		/// The method that will be executed when solving a problem.
		/// </summary>
		public void Solve()
		{
			var sw = new Stopwatch();
			sw.Start();
			var sol1 = SolvePart1();
			sw.Stop();
			var timeTakenSol1 = sw.ElapsedMilliseconds;
			Console.WriteLine($"Part 1 Solution: {sol1}\n\tTime Taken: {timeTakenSol1} MS.");

			sw.Restart();
			var sol2 = SolvePart2();
			sw.Stop();
			var timeTakenSol2 = sw.ElapsedMilliseconds;
			Console.WriteLine($"Part 2 Solution: {sol2}\n\tTime Taken: {timeTakenSol2} MS.");
		}
	}
}