//Q1.Implement a function that checks whether a given string is a palindrome or not.

fn is_palindrome(s: &str) -> bool {
    let mut reversed = s.chars().rev().collect::<String>();
    s == reversed
}


//Q2.Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn find(){
    let array=[2,3,4,5,5,6,6,7];        
    
    let mut i=0;                        
    let num=5;                          
    let length=array.len();             
    for i in 0..length{
        if array[i]==num {
            println!("The first index of {num}  is = {i}");
            break;
        }
    }
}

//Q3.Given a string of words, implement a function that returns the shortest word in the string.

fn shortest_word(words: &str) -> &str {
    let mut shortest = &words[..];

    for word in words.split_whitespace() {
        if word.len() < shortest.len() {
            shortest = word;
        }
    }

    shortest
}

//Q4 Implement a function that checks whether a given number is prime or not.

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

//Q5.Given a sorted array of integers, implement a function that returns the median of the array.

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        return f64::NAN;
    }
    let mid = len / 2;
    if len % 2 == 0 {
        let a = arr[mid - 1] as f64;
        let b = arr[mid] as f64;
        (a + b) / 2.0
    } else {
        arr[mid] as f64
    }
}

//Q6.Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = String::from(strs[0]);

    for i in 1..strs.len() {
        let mut j = 0;
        while j < prefix.len() && j < strs[i].len() && prefix.as_bytes()[j] == strs[i].as_bytes()[j] {
            j += 1;
        }
        prefix.truncate(j);
    }

    prefix
}
//Q7.Implement a function that returns the kth smallest element in a given array.

fn kth_smallest(nums: &mut [i32], k: usize) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut k_index = k - 1; // k-th smallest is (k-1)-th order statistic

    while start <= end {
        let mut pivot_index = partition(nums, start, end);

        if pivot_index == k_index {
            return nums[pivot_index];
        } else if pivot_index > k_index {
            end = pivot_index - 1;
        } else {
            start = pivot_index + 1;
            k_index -= pivot_index - start + 1;
        }
    }

    unreachable!()
}

fn partition(nums: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = nums[end];
    let mut pivot_index = start;

    for i in start..end {
        if nums[i] < pivot {
            nums.swap(i, pivot_index);
            pivot_index += 1;
        }
    }

    nums.swap(pivot_index, end);
    pivot_index
}

//Q8.Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let node = root.as_ref().unwrap().borrow();
    let left_depth = max_depth(node.left.clone());
    let right_depth = max_depth(node.right.clone());

    1 + std::cmp::max(left_depth, right_depth)
}

//Q9.Reverse a string in Rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

//Check if a number is prime in Rust
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

//Q10.Merge two sorted arrays in Rust

fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(a.len() + b.len());
    let mut a_idx = 0;
    let mut b_idx = 0;

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] < b[b_idx] {
            merged.push(a[a_idx]);
            a_idx += 1;
        } else {
            merged.push(b[b_idx]);
            b_idx += 1;
        }
    }

    merged.extend_from_slice(&a[a_idx..]);
    merged.extend_from_slice(&b[b_idx..]);

    merged
}

//Q11.Find the maximum subarray sum in Rust

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for i in 1..nums.len() {
        current_sum = std::cmp::max(nums[i], current_sum + nums[i]);
        max_sum = std::cmp::max(max_sum, current_sum);
    }

    max_sum
}
