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

console.log(fibonacci(10));