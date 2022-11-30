/*
    This is a simple example of a FizzBuzz program.
    It is not optimized for performance.
    It is not optimized for memory usage.
    It is not optimized for readability.
    It is not optimized for maintainability.
    It is not optimized for anything.
    It is just a simple example of a FizzBuzz program.
 */
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
