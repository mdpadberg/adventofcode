const input = require('fs').readFileSync('adventofcode-private/data/2020/2.txt', 'utf8').split('\n');
let counter = 0;

input.forEach(line => {
    const policy = parseLine(line);
    const actualNumberOfTimes = policy.password.split(policy.givenLetter).length - 1;
    if (inRange(actualNumberOfTimes, policy)) {
        counter++;
    }
})

function parseLine(line) {
    const partsOfPolicy = line.split(' ');
    const numberOfTimes = partsOfPolicy[0].split('-');
    return {
        minOfTimes: parseInt(numberOfTimes[0]),
        maxOfTimes: parseInt(numberOfTimes[1]),
        givenLetter: partsOfPolicy[1].replace(':', ''),
        password: partsOfPolicy[2]
    }
}

function inRange(actualNumberOfTimes, policy) {
    return actualNumberOfTimes >= policy.minOfTimes && actualNumberOfTimes <= policy.maxOfTimes;
}

console.log(counter);