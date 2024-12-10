const input = require("fs").readFileSync("adventofcode-private/data/2020/4.txt", "utf8");
const passports = input.split("\n\n").map((line) => line.split("\n").join(" "));
const requiredFields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

console.log(
  passports
    .filter((passport) => validateIfAllRequiredFieldsAreInPassport(passport))
    .length
);

function validateIfAllRequiredFieldsAreInPassport(passport) {
  let amountOfRequiredFields = 0;
  for (let i = 0; i < requiredFields.length; i++) {
    if (passport.includes(requiredFields[i])) {
      amountOfRequiredFields++;
    }
  }
  return amountOfRequiredFields === requiredFields.length;
}
