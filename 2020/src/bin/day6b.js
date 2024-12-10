const input = require('fs').readFileSync('adventofcode-private/data/2020/6.txt', 'utf8')
const answersPerPersonDevidedIntoGroups = input
    .split('\n\n')
    .map(line => line.split('\n'))
    .map(value => value.map(a => a.split('')))

console.log(answersPerPersonDevidedIntoGroups
    .map(group => filterOccurencesBasedOnAmountOfPersonsInGroup(group))
    .reduce((sum, current) => sum + current, 0))

function filterOccurencesBasedOnAmountOfPersonsInGroup(input) {
    const amountOfPersonsInGroup = input.length;
    const allValues = flatMapArray(input);
    const occurences = countOccurrencesArray(allValues)
    return occurences.filter(occurence => amountOfPersonsInGroup === parseInt(occurence)).length;
}

function flatMapArray(input) {
    return input.flatMap(a => Array.from(a))
}

function countOccurrencesArray(input) {
    return Object.entries(countOccurrencesAndMakeAnObject(input)).map((key, index) => key[1])
}

function countOccurrencesAndMakeAnObject(input) {
    return input.reduce(function (a, b) {
        a[b] = a[b] + 1 || 1
        return a;
    }, {});
}