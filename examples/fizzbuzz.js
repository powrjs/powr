function fizzbuzz(until_number) {
    let result = "";

    for (let i = 1; i <= until_number; i++) {
        if (i % 3 === 0) {
            result += "Fizz";
        } else if (i % 5 === 0) {
            result += "Buzz";
        } else {
            result += i;
        }

        result += "\n";
    }

    return result;
}

console.log(fizzbuzz(100));