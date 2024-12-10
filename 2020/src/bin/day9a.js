const input = require("fs")
  .readFileSync("adventofcode-private/data/2020/9.txt", "utf8")
  .split("\n")
  .map((line) => parseInt(line));

console.log(calculateInvalidPreamble(input, 25));

function calculateInvalidPreamble(numbers, preamble) {
  for (let i = preamble; i < numbers.length; i++) {
    if (!contains(numbers[i], new Set(numbers.slice(i - preamble, i)))) {
      return numbers[i];
    }
  }
}

function contains(sum, range) {
  return [...range.values()].some((number) => range.has(sum - number));
}
