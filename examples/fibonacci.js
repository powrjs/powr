// this is a simple example of a fibonacci sequence generator
// it is not optimized for performance
// it is not optimized for memory usage
// it is not optimized for readability
// it is not optimized for maintainability
// it is not optimized for anything
// it is just a simple example of a fibonacci sequence generator
// gotta love copilot
function fibonacci(number) {
    let result = 0;
    let previous = 0;
    let current = 1;

    for (let i = 0; i < number; i++) {
        result = previous + current;
        previous = current;
        current = result;
    }

    return result;
}

// usage of the fibonacci function
console.log(fibonacci(10));
