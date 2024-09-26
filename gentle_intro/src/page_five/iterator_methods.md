# Here’s a list of iterator methods in Rust:

 - Prepare list of small programs to learn all this functions.

```markdown

1. `next`
2. `collect`
3. `map`
4. `filter`
5. `filter_map`
6. `enumerate`
7. `peekable`
8. `peek`
9. `skip`
10. `skip_while`
11. `take`
12. `take_while`
13. `chain`
14. `cycle`
15. `zip`
16. `fold`
17. `for_each`
18. `partition`
19. `find`
20. `find_map`
21. `position`
22. `rposition`
23. `any`
24. `all`
25. `count`
26. `last`
27. `nth`
28. `max`
29. `min`
30. `max_by`
31. `min_by`
32. `max_by_key`
33. `min_by_key`
34. `sum`
35. `product`
36. `cmp`
37. `eq`
38. `ne`
39. `lt`
40. `le`
41. `gt`
42. `ge`
43. `by_ref`
44. `cloned`
45. `copied`
46. `flatten`
47. `fuse`
48. `inspect`
49. `rev`
50. `unzip`
51. `extend`
52. `intersperse`
53. `intersperse_with`
54. `is_sorted`
55. `is_sorted_by`
56. `is_sorted_by_key`
57. `flat_map`
58. `dedup`
59. `dedup_by`
60. `dedup_by_key`

```


## Problem Statement Topic wise.

Here are 20 problem statements designed to help you practice Rust iterator methods:

### Problem Set: Iterator Methods in Rust

1. **Basic Iteration & Collection**  
   Given a vector of integers, use an iterator to double each value and collect the results into a new vector.

2. **Filtering Values**  
   From a list of numbers, filter out all even numbers and collect the odd ones into a new vector.

3. **Mapping with Transformation**  
   Given a list of strings, transform each string to uppercase using the `map` method.

4. **Skipping and Taking Values**  
   Take the first 5 elements from a vector of integers, skipping the first 2.

5. **Chain Iterators**  
   Concatenate two vectors of integers into a single iterator using `chain` and collect them into a new vector.

6. **Cycle through Elements**  
   Given a small array of numbers, create an infinite iterator that cycles through the elements, and collect the first 10 values.

7. **Find a Value**  
   From a list of strings, find the first string that starts with the letter "R".

8. **Peekable Iterators**  
   Create a peekable iterator from a vector of integers. Use `peek` to look at the next element without advancing the iterator, and print it.

9. **Enumerating Elements**  
   Given a list of words, enumerate each word with its index in the list and print them.

10. **Zipping Two Iterators**  
    Given two vectors of equal length, zip them together to create a vector of tuples where each tuple contains one element from each vector.

11. **Folding Values**  
    Compute the product of all elements in a vector using the `fold` method.

12. **Reversing Iteration**  
    Reverse a vector of integers and collect the reversed elements into a new vector.

13. **Skipping While**  
    Skip all elements from a vector of integers until the first element greater than 10 is encountered, then collect the rest.

14. **Take While Condition**  
    Take elements from a vector of integers while the numbers are less than 50, and collect them into a new vector.

15. **Flat Mapping Nested Iterators**  
    Given a list of vectors, flatten them into a single iterator using `flat_map` and collect the result into a new vector.

16. **Finding Maximum and Minimum**  
    From a list of floats, find the maximum and minimum values using `max` and `min`.

17. **Count Elements**  
    Count the number of even numbers in a vector using the `count` method.

18. **Using Inspect to Debug**  
    Create a pipeline where you print each element of a vector before doubling it using the `inspect` method, then collect the results into a new vector.

19. **Partitioning Elements**  
    Partition a list of integers into two collections: one containing even numbers and the other containing odd numbers.

20. **Deduplication**  
    Given a list of integers with repeated values, use `dedup` to remove consecutive duplicates and collect the result into a new vector.


# Set of 50 questions to learn all the iterator stuff.

Here’s a set of **50 problem statements** to cover all the iterator methods from the list:

- [ ] Double each value in a vector of integers and collect them into a new vector.
- [ ] Filter out all even numbers from a vector and collect the odd ones.
- [ ] Convert a list of strings to uppercase using `map`.
- [ ] Skip the first 2 elements and take the next 5 from a vector of integers.
- [ ] Concatenate two vectors of integers using `chain` and collect them.
- [ ] Create a cycle from an array of numbers and collect the first 10 values.
- [ ] Find the first string that starts with "R" in a list of strings.
- [ ] Peek at the next element of a peekable iterator without advancing it.
- [ ] Enumerate a list of strings and print each string with its index.
- [ ]  Zip two vectors of integers and collect them into tuples.

- [ ] Sum up all the integers in a vector using `fold`.

- [ ] Reverse a vector of integers and collect the reversed elements.

- [ ] Skip elements from a vector until the first number greater than 10.

- [ ] Take elements from a vector while they are less than 50.

- [ ] Flatten a list of vectors of integers into a single vector.

- [ ] Find the maximum value from a list of floats.

- [ ] Find the minimum value from a list of integers.

- [ ] Count the number of even numbers in a vector.

- [ ] Print each element of a vector before squaring it using `inspect`.

- [ ] Partition a list of integers into even and odd numbers.

- [ ] Remove consecutive duplicates from a vector using `dedup`.

- [ ] Use `peekable` and `peek` to check if the next element of a vector is greater than 10.

- [ ] Collect the first 3 unique numbers from a vector using `take`.

- [ ] Compare two vectors to check if they are equal using `eq`.

- [ ] Check if all elements in a vector are positive numbers using `all`.

- [ ] Check if any element in a vector is greater than 100 using `any`.

- [ ] Find the position of the first element greater than 20 in a vector.

- [ ] Find the last element in a vector that is divisible by 5.

- [ ] Use `rposition` to find the last index of an odd number in a vector.

- [ ] Compute the product of all numbers in a vector using `product`.

- [ ] Sum all the odd numbers in a vector using `sum`.

- [ ] Count the occurrences of a specific string in a list of strings using `filter`.

- [ ] Clone all elements in a vector of integers using `cloned`.

- [ ] Use `by_ref` to iterate through the first half of a vector without consuming the iterator.

- [ ] Collect a vector of integers into a tuple of two vectors by partitioning even and odd numbers.

- [ ] Find the first string in a list that contains the letter "a" using `find_map`.

- [ ] Skip numbers from a vector while they are less than 10 using `skip_while`.

- [ ] Combine two lists of integers using `zip` and add their values pairwise.

- [ ] Remove the smallest element from a list of integers using `min_by`.

- [ ] Find the element with the longest length in a list of strings using `max_by_key`.

- [ ] Interleave elements of two lists using `intersperse`.

- [ ] Check if a list of integers is sorted using `is_sorted`.

- [ ] Check if a list of tuples is sorted by their second element using `is_sorted_by`.

- [ ] Filter out `None` values from a list of `Option` integers using `filter_map`.

- [ ] Get the nth element from a vector of integers.

- [ ] Convert a vector of `Option` integers to a vector of integers by unwrapping the values using `flatten`.

- [ ] Use `fuse` to create an iterator that stops once exhausted.

- [ ] Unzip a list of tuples into two separate vectors.

- [ ] Count how many elements are greater than 5 in a vector using `count`.

- [ ] Collect a vector of strings into a tuple of words shorter and longer than 5 characters using `partition`.

These problems will give you solid practice with Rust iterators. Let me know if you'd like help with any specific ones!