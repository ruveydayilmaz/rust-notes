## Building a Custom Filtering Function in Rust

### Steps
1. Define a struct called FilterCondition with a single field of the desired type for filtering.
2. Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.
3. Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
4. In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.
5. Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
6. Print the filtered result to the console.
7. Call the calculate function with the created Operation enum instance.
8.Compile and run the program to test its functionality.

### Checklist
✅ Create a new Rust project and open the main.rs file.

✅ Define the FilterCondition struct with the desired type for filtering.

✅ Implement the is_match method on the FilterCondition struct.

✅ Define the custom_filter function to filter elements based on the condition.

✅ Create a collection and a FilterCondition object in the main function.

✅ Call the custom_filter function and store the result.

✅ Print the filtered result to the console.

✅ Compile and run the program to test its functionality.