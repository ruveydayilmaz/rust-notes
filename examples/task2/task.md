## Create a simple calculator using enums and pattern matching

### Steps
1. Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.
2. Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.
3. Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.
4. In the main function, prompt the user to input the first number, the operation to be performed, and the second number.
5. Parse the user input into appropriate variables.
6. Create an Operation enum instance with the parsed input values.
7. Call the calculate function with the created Operation enum instance.
8. Print the result to the console.
9. Compile and run the program to ensure it works as expected.

### Checklist
✅ Define the Operation enum with the appropriate variants and values.

✅ Write the calculate function signature.

✅ Implement the calculate function using pattern matching to perform the appropriate arithmetic operation.

✅ Prompt the user to input the first number, operation, and second number.

✅ Parse the user input into appropriate variables.

✅ Create an Operation enum instance with the parsed input values.

✅ Call the calculate function with the created Operation enum instance.

✅ Print the result to the console.

✅ Compile and run the program to test its functionality.