const input = require('fs').readFileSync('adventofcode-private/data/2020/6.txt', 'utf8')
const answersPerGroup = input
    .split('\n\n')
    .map(line => line.split('\n').join(''))
    .map(value => value.split(''))
const uniqueAnswersPerGroup = answersPerGroup.map(answers => new Set(answers));

console.log(uniqueAnswersPerGroup.map(set => set.size).reduce((sum, current) => sum + current, 0))
