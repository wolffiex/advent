function part2() {
    const [initial, rules] = getInput().split("\n\n");
    let ruleMap = Object.fromEntries(
        rules.split("\n").map(line => line.split(" -> ")));
    console.log(ruleMap);

    let sequence = generateSequence(initial, ruleMap);
    let freq = new Map();
    let it = sequence.next();
    while(!it.done) {
        let c = it.value
        let v = freq.get(c) || 0;
        freq.set(c, v+1);
        it = sequence.next();
    }

    console.log(sequence)

    console.log(">", freq)
    
    
}

function* generateSequence(initial, ruleMap) {
    function* generateExpansion(a, b, depth) {
        let e = ruleMap[a + b];
        if (depth > 25) {
            yield e;
        } else {
            yield* generateExpansion(a, e, depth +1);
            yield e;
            yield* generateExpansion(e, b, depth +1);
        }
    }

    yield initial[0];
    for (var i = 1; i< initial.length; i++) {
        const [a,b] = initial.substring(i-1);
        yield* generateExpansion(a, b, 1);
        yield b;
    }
}

function getInput() {
    return `NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`;
}

part2(getInput());
