const input = require('fs').readFileSync('adventofcode-private/data/2020/3.txt', 'utf8').split('\n');

console.log(calculateAmountOfTrees(3,1));

function calculateAmountOfTrees(incrementColumn, incrementRow) {
    let amountOfTrees = 0;
    for (let i = incrementRow, j = incrementColumn; i < input.length; i += incrementRow) {
        if (checkIfCoordinatesContainATree(i, j)) {
            amountOfTrees++;
        }
        if (checkIfCoordinatesGoOutsideOfTheMap(j, i, incrementColumn)) {
            j = resetCounterToLeftSideOfTheMap(j, i, incrementColumn);
        } else {
            j += incrementColumn;
        }
    }
    return amountOfTrees;
}

function resetCounterToLeftSideOfTheMap(j, i, incrementColumn) {
    return (j + incrementColumn) % input[i].length;
}

function checkIfCoordinatesGoOutsideOfTheMap(j, i, incrementColumn) {
    return (j + incrementColumn) >= input[i].length;
}

function checkIfCoordinatesContainATree(i, j) {
    return input[i].charAt(j) === '#';
}