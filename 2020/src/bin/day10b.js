const input = require("fs")
.readFileSync("adventofcode-private/data/2020/10.txt", "utf8")
.split("\n")
.map((value) => parseInt(value))
.sort((a, b) => a - b)
input.unshift(0);
input.push(input[input.length - 1] + 3);

console.log(calculateAmountOfOptions());

function calculateAmountOfOptions() {
  const amountOfSteps = [1];
  for (let i = 0; i < input.length; i++) {
    for (let j = i + 1; input[j] <= input[i] + 3; j++) {
      if(amountOfSteps[j]) {
        amountOfSteps[j] = amountOfSteps[j] + amountOfSteps[i];
      } else {
        amountOfSteps[j] = 0 + amountOfSteps[i];
      }
    }
  }
  return amountOfSteps[input.length - 1];
}
