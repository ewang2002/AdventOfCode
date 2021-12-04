class Node {
    constructor() {
        this.childNodes = [];
        this.metaData = [];
    }
}

// Part 1

// Parent function
function setNodes(input) {
    let node = new Node();
    run1(input, node);
    return node;
}

// "node" refers to the current node to deal with.
// The input
function run1(input, node) {
    // No more input to process.
    if (input.length === 0) return;

    // Get child node + metadata count.
    let numChildNodes = input.shift();
    let numMetaData = input.shift();

    // If no child nodes, then put metadata in the current node and leave
    if (numChildNodes === 0) {
        while (numMetaData > 0) {
            node.metaData.push(input.shift());
            numMetaData--;
        }
        return;
    }

    // Otherwise, create a new node for each child node then attach to the current node.
    while (numChildNodes > 0) {
        const newNode = new Node();
        run1(input, newNode);
        node.childNodes.push(newNode);
        numChildNodes--;
    }

    // Then, add the meta data to the current node.
    while (numMetaData > 0) {
        node.metaData.push(input.shift());
        numMetaData--;
    }
}

// Part 2

function secondPart(node) {
    let sum = 0;
    for (const md of node.metaData) {
        const res = run2(node, md);
        sum += res;
    }
    return sum;
}

function run2(node, idx) {
    // Base Case
    const realIdx = idx - 1;
    if (idx === 0 || realIdx >= node.childNodes.length) {
        return 0;
    }


    if (node.childNodes[realIdx].childNodes.length === 0) {
        return node.childNodes[realIdx].metaData.reduce((a, b) => a + b, 0);
    }

    let sum = 0;
    for (const i of node.childNodes[realIdx].metaData)
        sum += run2(node.childNodes[realIdx], i);
    return sum;
}

const input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".split(" ").map(x => Number.parseInt(x, 10));
//const input = document.body.innerText.split(" ").map(x => Number.parseInt(x, 10));
const res = setNodes(input);
const pt2 = secondPart(res);
console.log(pt2);