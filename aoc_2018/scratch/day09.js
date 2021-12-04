function day09Part1(numPlayers, lastMarble) {
	const marbles = [0];
	const players = [];
	for (let i = 0; i < numPlayers; i++) players.push(0); 
	
	let currentMarbleIndex = 0;
	// zero-indexed
	let currentPlayer = 0;
	// one-indexed
	let currentMarble = 1;
	while (currentMarble !== lastMarble + 1) {
		currentPlayer %= numPlayers; 

		if (currentMarble % 23 === 0) {
			// Go 7 elements back
			currentMarbleIndex -= 7;
			if (currentMarbleIndex < 0) currentMarbleIndex = marbles.length - Math.abs(currentMarbleIndex);
			// Then take that element away.
			let score = marbles.splice(currentMarbleIndex === marbles.length - 1 ? 0 : currentMarbleIndex, 1)[0] + currentMarble;
			players[currentPlayer] += score;
			console.log(`${currentPlayer} => ${players[currentPlayer]}`);
			
			currentPlayer++; 
			currentMarble++; 

			continue;
		}

		currentMarbleIndex = addToMarbleArr(marbles, currentMarbleIndex, currentMarble); 
		currentMarble++; 
		currentPlayer++; 
	}

	console.log(players);
	console.log(marbles);
	return Math.max(...players); 
}


function addToMarbleArr(arr, idx, num) {
    if (idx + 2 < arr.length) {
        arr.splice(idx + 2, 0, num);
        return idx + 2;
    }

    if (idx === arr.length - 2) {
        arr.push(num);
        return arr.length - 1;
    }

    // assume idx === arr.length - 1
    arr.splice(1, 0, num);
    return 1;
}
