// just scratch work for day 18
// noinspection JSUnusedGlobalSymbols

const LEFT_BRACE = -1;
const RIGHT_BRACE = -2;
const COMMA = -3;

function parseLine(input: string): number[] {
    const line: number[] = [];
    const a = input.split("");
    for (const c of a) {
        if (c === "[") {
            line.push(LEFT_BRACE);
            continue;
        }

        if (c === "]") {
            line.push(RIGHT_BRACE);
            continue;
        }

        if (c === ",") {
            line.push(COMMA);
            continue;
        }

        const n = parseInt(c, 10);
        line.push(n);
    }

    return line;
}

function add(a: number[], b: number[]): number[] {
    const newLine: number[] = [];
    newLine.push(LEFT_BRACE);
    newLine.push(...a);
    newLine.push(COMMA);
    newLine.push(...b);
    newLine.push(RIGHT_BRACE);
    return newLine;
}

function explode(line: number[]): boolean {
    let b: number = 0;
    for (let i = 0; i < line.length; i++) {
        if (line[i] === LEFT_BRACE) {
            b++;
        }

        if (line[i] === RIGHT_BRACE) {
            b--;
        }

        // Must be a number
        // Note that outer braces don't count
        if (b < 5) {
            continue;
        }

        // Must be a pair
        if (line[i] === COMMA && line[i - 1] >= 0 && line[i + 1] >= 0) {
            const beforeNum = line[i - 1];
            const afterNum = line[i + 1];
            for (let j = i - 2; j >= 0; j--) {
                if (line[j] < 0) {
                    continue;
                }

                line[j] += beforeNum;
                break;
            }

            for (let j = i + 2; j < line.length; j++) {
                if (line[j] < 0) {
                    continue;
                }

                line[j] += afterNum;
                break;
            }

            line.splice(i - 2, 5, 0);
            return true;
        }
    }

    return false;
}

function split(line: number[]): boolean {
    for (let i = 0; i < line.length; i++) {
        if (line[i] === LEFT_BRACE || line[i] === RIGHT_BRACE || line[i] === COMMA) {
            continue;
        }

        if (line[i] < 10) {
            continue;
        }

        const left = Math.floor(line[i] / 2);
        const right = line[i] - left;
        line.splice(i, 1);
        line.splice(i, 0, LEFT_BRACE, left, COMMA, right, RIGHT_BRACE);
        return true;
    }

    return false;
}

function getStringRepresentation(line: number[]): string {
    const s: string[] = [];
    for (const c of line) {
        switch (c) {
            case LEFT_BRACE: {
                s.push("[");
                break;
            }
            case RIGHT_BRACE: {
                s.push("]");
                break;
            }
            case COMMA: {
                s.push(",");
                break;
            }
            default: {
                s.push(c.toString());
                break;
            }
        }
    }

    return s.join("");
}

function calculateMagnitude(line: number[]): number {
    while (flatten(line)) { }
    return line[0];
}

function flatten(line: number[]): boolean {
    for (let i = 0; i < line.length; i++) {
        if (line[i] === COMMA && line[i - 1] >= 0 && line[i + 1] >= 0) {
            const left = line[i - 1];
            const right = line[i + 1];
            line.splice(i - 2, 5, 3 * left + 2 * right);
            return true;
        }
    }

    return false;
}

function run(problems: string[]): number {
    let start = parseLine(problems[0]);
    processLine(start);

    for (let i = 1; i < problems.length; i++) {
        start = add(start, parseLine(problems[i]));
        processLine(start);
    }

    return calculateMagnitude(start);
}

function processLine(line: number[]): void {
    while (true) {
        if (explode(line)) {
            continue;
        }

        if (split(line)) {
            continue;
        }

        break;
    }
}

function getHighestMagnitude(problems: string[]): number {
    let max = 0;
    for (let i = 0; i < problems.length; i++) {
        for (let j = 0; j < problems.length; j++) {
            if (i === j) {
                continue;
            }

            const a = run([problems[i], problems[j]]);
            const b = run([problems[j], problems[i]]);

            if (a > max) {
                max = a;
            }

            if (b > max) {
                max = b;
            }
        }
    }

    return max;
}