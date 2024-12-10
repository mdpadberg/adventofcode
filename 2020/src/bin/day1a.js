const input = require('fs').readFileSync('adventofcode-private/data/2020/1.txt', 'utf8').split('\n');
const output = new Set();

for (let i = 0; i < input.length; i++) {
	for (let j= 0; j < input.length; j++) {
		if(parseInt(input[i]) + parseInt(input[j]) === 2020) {
            output.add(input[i].toString() * input[j].toString());
		}
	}
}

console.log(output);