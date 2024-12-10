const input = require('fs').readFileSync('adventofcode-private/data/2020/5.txt', 'utf8').split('\n');
const seatSpecification = {
    "rowRangeLow": 0,
    "rowRangeHigh": 127,
    "columnRangeLow": 0,
    "columnRangeHigh": 7,
    "calculateSeatId": function () {
        return this.rowRangeLow * 8 + this.columnRangeLow;
    }
}

console.log(Math.max(...input
    .map(seatInBinaryFormat => calculateSeatBasedOnBinaryFormat(seatInBinaryFormat))
    .map(seat => seat.calculateSeatId())))

function calculateSeatBasedOnBinaryFormat(input) {
    const characters = input.split('');
    let currentSeat = seatSpecification;
    for (let i = 0; i < characters.length; i++) {
        currentSeat = applyBinaryInput(currentSeat, characters[i]);
    }
    return currentSeat;
}

function applyBinaryInput(currentSeat, character) {
    switch (character) {
        case 'F':
            return lowerHalfOfRow(currentSeat);
        case 'B':
            return upperHalfOfRow(currentSeat);
        case 'L':
            return lowerHalfOfColumn(currentSeat);
        case 'R':
            return upperHalfOfColumn(currentSeat);
        default:
            console.log('dont know character: ', character);
    }
}

function lowerHalfOfRow(seat) {
    const difference = seat.rowRangeHigh - seat.rowRangeLow;
    const amountToLowerWith = Math.ceil(difference / 2);
    return {
        ...seat,
        "rowRangeHigh": seat.rowRangeHigh - amountToLowerWith,
    }
}

function upperHalfOfRow(seat) {
    const difference = seat.rowRangeHigh - seat.rowRangeLow;
    const amountToIncreaseWith = Math.ceil(difference / 2);
    return {
        ...seat,
        "rowRangeLow": seat.rowRangeLow + amountToIncreaseWith,
    };
}

function lowerHalfOfColumn(seat) {
    const difference = seat.columnRangeHigh - seat.columnRangeLow;
    const amountToLowerWith = Math.ceil(difference / 2);
    return {
        ...seat,
        "columnRangeHigh": seat.columnRangeHigh - amountToLowerWith,
    };
}

function upperHalfOfColumn(seat) {
    const difference = seat.columnRangeHigh - seat.columnRangeLow;
    const amountToIncreaseWith = Math.ceil(difference / 2);
    return {
        ...seat,
        "columnRangeLow": seat.columnRangeLow + amountToIncreaseWith,
    };
}