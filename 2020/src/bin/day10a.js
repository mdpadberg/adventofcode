const input = require("fs")
  .readFileSync("adventofcode-private/data/2020/10.txt", "utf8")
  .split("\n")
  .map((value) => parseInt(value))
  .sort((a, b) => a - b);

console.log(calculateJoltDifferences(input));

function calculateJoltDifferences(data) {
  let oneJoltDifferences = 1;
  let threeJoltDifferences = 1;
  for (let i = 0; i < data.length; i++) {
    const values = getValues(data, i);
    if (values.difference === 1) {
      oneJoltDifferences++;
    } else if (values.difference === 3) {
      threeJoltDifferences++;
    }
  }
  return {
    oneJoltDifferences,
    threeJoltDifferences,
    oneJoltTimesThreeJolt: (oneJoltDifferences * threeJoltDifferences)
  };
}

function getValues(data, i) {
  const currentValue = data[i];
  const nextValue = data[i + 1];
  if (currentValue && nextValue) {
    return {
      current: data[i],
      next: data[i + 1],
      difference: nextValue - currentValue,
    };
  } else {
    return {
      current: data[i],
      next: data[i + 1],
      difference: undefined,
    };
  }
}
