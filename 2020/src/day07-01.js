const input = require('fs').readFileSync('2020/data/7.txt', 'utf8').replace(/\./g, '').split('\n');

console.log(new Set(checkAmountOfBagsThatCanContainBagName('shiny gold')))

function checkAmountOfBagsThatCanContainBagName(bagName) {
  const initialInput = input.filter(line => doesLineContainBagName(line, bagName));
  const recursive = initialInput.map(line => checkAmountOfBagsThatCanContainBagName(extractBagNameFromInputLine(line)));
  return initialInput.concat(recursive).flat();
}

function doesLineContainBagName(line, bagName) {
  return line.match('. ' + bagName + ' bag') || line.match('. ' + bagName + ' bags');
}

function extractBagNameFromInputLine(line) {
  return line.substring(0, line.search('bags') - 1);
}

