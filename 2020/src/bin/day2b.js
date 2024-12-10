const input = require('fs').readFileSync('adventofcode-private/data/2020/2.txt', 'utf8').split('\n');
let counter = 0;

input.forEach(line => {
    const policy = parseLine(line);
    if (givenLetterOnFirstPosition(policy) ^ givenLetterOnSecondPosition(policy)) {
        counter++;
    }
})

function parseLine(line) {
    const partsOfPolicy = line.split(' ');
    const positions = partsOfPolicy[0].split('-');
    return {
        firstPosition: parseInt(positions[0]),
        secondPosition: parseInt(positions[1]),
        givenLetter: partsOfPolicy[1].replace(':', ''),
        password: partsOfPolicy[2]
    }
}

function givenLetterOnFirstPosition(policy) {
    return policy.givenLetter === policy.password.charAt(policy.firstPosition - 1);
}

function givenLetterOnSecondPosition(policy) {
    return policy.givenLetter === policy.password.charAt(policy.secondPosition - 1);
}

console.log(counter);