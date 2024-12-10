const input = require("fs").readFileSync("adventofcode-private/data/2020/4.txt", "utf8");
const passports = input.split("\n\n").map((line) => line.split("\n").join(" "));
const requiredFields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
const validationRules = new Map([
  ["byr", validateBirthYear],
  ["iyr", validateIssueYear],
  ["eyr", validateExpirationYear],
  ["hgt", validateHeight],
  ["hcl", validateHairColor],
  ["ecl", validateEyeColor],
  ["pid", validatePassportId],
  ["cid", validateCountryId],
]);

console.log(passports
    .filter((passport) => validateIfAllRequiredFieldsAreInPassport(passport))
    .filter((passport) => validateContentOfRequiredFielsInPassport(passport))
    .length
)

function validateIfAllRequiredFieldsAreInPassport(passport) {
  let amountOfRequiredFields = 0;
  for (let i = 0; i < requiredFields.length; i++) {
    if (passport.includes(requiredFields[i])) {
      amountOfRequiredFields++;
    }
  }
  return amountOfRequiredFields === requiredFields.length;
}

function validateContentOfRequiredFielsInPassport(passport) {
  return !passport
    .trim()
    .split(" ")
    .map((keyValue) => keyValuePassportInformation(keyValue.split(":")))
    .includes(false);
}

function keyValuePassportInformation(keyValue) {
  if (keyValue.length == 2 && validationRules.has(keyValue[0])) {
    const validationRule = validationRules.get(keyValue[0]);
    if (validationRule) {
      return validationRule(keyValue[1]);
    }
  }
  return false;
}

function validateBirthYear(input) {
  return input >= 1920 && input <= 2002;
}

function validateIssueYear(input) {
  return input >= 2010 && input <= 2020;
}

function validateExpirationYear(input) {
  return input >= 2020 && input <= 2030;
}

function validateHeight(input) {
  if (input.includes("cm")) {
    const height = parseInt(input.replace("cm", ""));
    return height >= 150 && height <= 193;
  }
  if (input.includes("in")) {
    const height = parseInt(input.replace("in", ""));
    return height >= 59 && height <= 76;
  }
  return false;
}

function validateHairColor(input) {
  return (
    input.length === 7 &&
    input.charAt(0) === "#" &&
    RegExp(/^[0-9a-f]+$/).test(input.substr(1))
  );
}

function validateEyeColor(input) {
  return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(input);
}

function validatePassportId(input) {
  return input.length === 9 && RegExp(/^[0-9]+$/).test(input);
}

function validateCountryId(input) {
  return true;
}
