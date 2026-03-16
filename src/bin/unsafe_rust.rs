// Rust Tutorial #30: Unsafe Rust — When and How to Use It Safely
// Demonstrates raw pointers, unsafe functions, unsafe traits,
// FFI patterns, and guidelines for using unsafe correctly.

use std::fmt;

// ========================================================
// Concept 1: Raw Pointers
// ========================================================

fn raw_pointer_basics() {
    let x = 42;
    let r1 = &x as *const i32; // immutable raw pointer
    let mut y = 10;
    let r2 = &mut y as *mut i32; // mutable raw pointer

    // Creating raw pointers is safe. Dereferencing them is unsafe.
    unsafe {
        println!("r1 points to: {}", *r1);
        println!("r2 points to: {}", *r2);
        *r2 = 20;
        println!("r2 after write: {}", *r2);
    }
}

fn pointer_from_address() -> i32 {
    let mut value = 99;
    let ptr = &mut value as *mut i32;

    unsafe {
        *ptr += 1;
        *ptr
    }
}

fn pointer_arithmetic() -> Vec<i32> {
    let data = vec![10, 20, 30, 40, 50];
    let ptr = data.as_ptr();
    let mut results = Vec::new();

    unsafe {
        for i in 0..data.len() {
            results.push(*ptr.add(i));
        }
    }

    results
}

// ========================================================
// Concept 2: Unsafe Functions
// ========================================================

/// Splits a mutable slice at the given index.
/// The caller must ensure mid <= slice.len().
unsafe fn split_at_unchecked(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    // We need unsafe because we create two mutable references
    // to different parts of the same slice.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// Safe wrapper around the unsafe split function.
fn split_at_safe(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid <= slice.len(), "mid out of bounds");
    unsafe { split_at_unchecked(slice, mid) }
}

// ========================================================
// Concept 3: Implementing a Simple Unsafe Data Structure
// ========================================================

struct SimpleVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SimpleVec<T> {
    fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        Self {
            ptr,
            len: 0,
            capacity,
        }
    }

    fn push(&mut self, value: T) {
        if self.len >= self.capacity {
            self.grow();
        }
        unsafe {
            self.ptr.add(self.len).write(value);
        }
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe { Some(self.ptr.add(self.len).read()) }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        unsafe { Some(&*self.ptr.add(index)) }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };

        let new_layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
        let new_ptr = if self.capacity == 0 {
            unsafe { std::alloc::alloc(new_layout) as *mut T }
        } else {
            let old_layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                std::alloc::realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut T
            }
        };

        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T> Drop for SimpleVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            // Drop all elements
            for i in 0..self.len {
                unsafe {
                    self.ptr.add(i).drop_in_place();
                }
            }
            // Free memory
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

// ========================================================
// Concept 4: Unsafe Traits
// ========================================================

/// A trait that promises the type can be safely zeroed.
/// Implementing this incorrectly can cause undefined behavior.
unsafe trait Zeroable {
    fn zeroed() -> Self;
}

unsafe impl Zeroable for i32 {
    fn zeroed() -> Self {
        0
    }
}

unsafe impl Zeroable for u32 {
    fn zeroed() -> Self {
        0
    }
}

unsafe impl Zeroable for f64 {
    fn zeroed() -> Self {
        0.0
    }
}

unsafe impl Zeroable for bool {
    fn zeroed() -> Self {
        false
    }
}

fn create_zeroed_array<T: Zeroable, const N: usize>() -> [T; N] {
    std::array::from_fn(|_| T::zeroed())
}

// ========================================================
// Concept 5: FFI — Calling C Functions
// ========================================================

// In real FFI, you'd link against a C library.
// This simulates the pattern.

/// Simulated C-style string handling
struct CString {
    data: Vec<u8>,
}

impl CString {
    fn new(s: &str) -> Self {
        let mut data: Vec<u8> = s.bytes().collect();
        data.push(0); // null terminator
        Self { data }
    }

    fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    fn len(&self) -> usize {
        self.data.len() - 1 // exclude null terminator
    }
}

/// Simulated C-style string length (like strlen)
unsafe fn c_strlen(ptr: *const u8) -> usize {
    let mut len = 0;
    unsafe {
        while *ptr.add(len) != 0 {
            len += 1;
        }
    }
    len
}

/// Simulated C-style string comparison (like strcmp)
unsafe fn c_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let ca = unsafe { *a.add(i) };
        let cb = unsafe { *b.add(i) };
        if ca != cb {
            return ca as i32 - cb as i32;
        }
        if ca == 0 {
            return 0;
        }
        i += 1;
    }
}

/// Simulated C-style memory copy (like memcpy)
unsafe fn c_memcpy(dst: *mut u8, src: *const u8, n: usize) {
    for i in 0..n {
        unsafe {
            *dst.add(i) = *src.add(i);
        }
    }
}

// ========================================================
// Concept 6: Safe Abstraction over Unsafe
// ========================================================

/// A fixed-size array that allows interior mutability
/// through raw pointers (demonstrating the pattern).
struct UnsafeCell<T> {
    value: std::cell::UnsafeCell<T>,
}

impl<T> UnsafeCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: std::cell::UnsafeCell::new(value),
        }
    }

    fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }

    fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }
}

// Safe interface hiding the unsafe implementation
struct SharedCounter {
    count: UnsafeCell<i32>,
}

impl SharedCounter {
    fn new() -> Self {
        Self {
            count: UnsafeCell::new(0),
        }
    }

    fn increment(&self) {
        let current = *self.count.get();
        self.count.set(current + 1);
    }

    fn get(&self) -> i32 {
        *self.count.get()
    }
}

// ========================================================
// Concept 7: Transmute and Type Punning
// ========================================================

fn float_to_bits(f: f32) -> u32 {
    unsafe { std::mem::transmute(f) }
}

fn bits_to_float(bits: u32) -> f32 {
    unsafe { std::mem::transmute(bits) }
}

fn is_nan_check(f: f32) -> bool {
    let bits = float_to_bits(f);
    let exponent = (bits >> 23) & 0xFF;
    let mantissa = bits & 0x7FFFFF;
    exponent == 0xFF && mantissa != 0
}

fn inspect_float_bits(f: f32) -> (bool, u32, u32) {
    let bits = float_to_bits(f);
    let sign = bits >> 31 == 1;
    let exponent = (bits >> 23) & 0xFF;
    let mantissa = bits & 0x7FFFFF;
    (sign, exponent, mantissa)
}

// ========================================================
// Concept 8: Union Types (C-compatible)
// ========================================================

#[repr(C)]
union NumberUnion {
    integer: i64,
    float: f64,
    bytes: [u8; 8],
}

fn demonstrate_union() {
    let mut num = NumberUnion { integer: 0 };

    // Writing to one field and reading from another is unsafe
    num.float = 3.14;
    let bytes = unsafe { num.bytes };
    let as_int = unsafe { num.integer };

    println!("Float 3.14 as bytes: {:?}", bytes);
    println!("Float 3.14 as integer: {}", as_int);
}

// ========================================================
// Concept 9: Static Mutable Variables
// ========================================================

static mut GLOBAL_COUNTER: i32 = 0;

fn increment_global() {
    unsafe {
        GLOBAL_COUNTER += 1;
    }
}

fn get_global() -> i32 {
    unsafe { GLOBAL_COUNTER }
}

// ========================================================
// Concept 10: Safe Wrapper Pattern
// ========================================================

/// A non-empty vector — guarantees at least one element.
struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

impl<T: Clone + fmt::Debug> NonEmptyVec<T> {
    fn new(first: T) -> Self {
        Self {
            first,
            rest: Vec::new(),
        }
    }

    fn from_vec(v: Vec<T>) -> Option<Self> {
        if v.is_empty() {
            return None;
        }
        let mut iter = v.into_iter();
        let first = iter.next().unwrap(); // safe: we checked non-empty
        let rest: Vec<T> = iter.collect();
        Some(Self { first, rest })
    }

    fn push(&mut self, value: T) {
        self.rest.push(value);
    }

    fn first(&self) -> &T {
        &self.first
    }

    fn last(&self) -> &T {
        self.rest.last().unwrap_or(&self.first)
    }

    fn len(&self) -> usize {
        1 + self.rest.len()
    }

    fn to_vec(&self) -> Vec<T> {
        let mut v = vec![self.first.clone()];
        v.extend(self.rest.clone());
        v
    }
}

fn main() {
    println!("=== Unsafe Rust Demo ===\n");

    // Demo 1: Raw pointers
    println!("--- Raw Pointers ---");
    raw_pointer_basics();
    println!("Pointer from address: {}", pointer_from_address());
    println!("Pointer arithmetic: {:?}", pointer_arithmetic());

    println!();

    // Demo 2: Unsafe functions
    println!("--- Unsafe Functions ---");
    let mut data = vec![1, 2, 3, 4, 5];
    let (left, right) = split_at_safe(&mut data, 3);
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);

    println!();

    // Demo 3: SimpleVec
    println!("--- SimpleVec ---");
    let mut v = SimpleVec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("Length: {}, Capacity: {}", v.len(), v.capacity());
    println!("Get(1): {:?}", v.get(1));
    println!("Pop: {:?}", v.pop());
    println!("Length after pop: {}", v.len());

    println!();

    // Demo 4: Unsafe traits
    println!("--- Unsafe Traits ---");
    let zeros_i32: [i32; 5] = create_zeroed_array();
    let zeros_f64: [f64; 3] = create_zeroed_array();
    println!("Zeroed i32: {:?}", zeros_i32);
    println!("Zeroed f64: {:?}", zeros_f64);

    println!();

    // Demo 5: FFI simulation
    println!("--- FFI Simulation ---");
    let s1 = CString::new("Hello");
    let s2 = CString::new("Hello");
    let s3 = CString::new("World");

    unsafe {
        println!("strlen(\"Hello\"): {}", c_strlen(s1.as_ptr()));
        println!("strcmp(Hello, Hello): {}", c_strcmp(s1.as_ptr(), s2.as_ptr()));
        println!("strcmp(Hello, World): {}", c_strcmp(s1.as_ptr(), s3.as_ptr()));
    }

    let mut dst = vec![0u8; 5];
    unsafe {
        c_memcpy(dst.as_mut_ptr(), s1.as_ptr(), 5);
    }
    println!("memcpy result: {:?}", std::str::from_utf8(&dst).unwrap());

    println!();

    // Demo 6: Safe abstraction
    println!("--- Safe Abstraction ---");
    let counter = SharedCounter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("Counter: {}", counter.get());

    println!();

    // Demo 7: Transmute
    println!("--- Transmute ---");
    let f: f32 = 1.0;
    let bits = float_to_bits(f);
    println!("1.0f32 as bits: {:#010X}", bits);
    println!("Back to float: {}", bits_to_float(bits));

    let (sign, exp, mantissa) = inspect_float_bits(1.0);
    println!("1.0: sign={}, exponent={}, mantissa={}", sign, exp, mantissa);

    let (sign, exp, mantissa) = inspect_float_bits(-2.5);
    println!("-2.5: sign={}, exponent={}, mantissa={}", sign, exp, mantissa);

    println!("is_nan(NaN): {}", is_nan_check(f32::NAN));
    println!("is_nan(1.0): {}", is_nan_check(1.0));

    println!();

    // Demo 8: Union
    println!("--- Union ---");
    demonstrate_union();

    println!();

    // Demo 9: Static mut
    println!("--- Static Mutable ---");
    increment_global();
    increment_global();
    increment_global();
    println!("Global counter: {}", get_global());

    println!();

    // Demo 10: Safe wrapper
    println!("--- NonEmptyVec ---");
    let nev = NonEmptyVec::from_vec(vec![1, 2, 3]).unwrap();
    println!("First: {}", nev.first());
    println!("Last: {}", nev.last());
    println!("Length: {}", nev.len());
    println!("As vec: {:?}", nev.to_vec());

    let empty = NonEmptyVec::<i32>::from_vec(vec![]);
    println!("Empty: {:?}", empty.is_none());

    println!("\n=== All demos completed! ===");
}

// ========================================================
// Tests
// ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Raw Pointers ---

    #[test]
    fn test_pointer_from_address() {
        assert_eq!(pointer_from_address(), 100);
    }

    #[test]
    fn test_pointer_arithmetic() {
        let result = pointer_arithmetic();
        assert_eq!(result, vec![10, 20, 30, 40, 50]);
    }

    // --- Split ---

    #[test]
    fn test_split_at_safe() {
        let mut data = vec![1, 2, 3, 4, 5];
        let (left, right) = split_at_safe(&mut data, 3);
        assert_eq!(left, &[1, 2, 3]);
        assert_eq!(right, &[4, 5]);
    }

    #[test]
    fn test_split_at_beginning() {
        let mut data = vec![1, 2, 3];
        let (left, right) = split_at_safe(&mut data, 0);
        assert!(left.is_empty());
        assert_eq!(right, &[1, 2, 3]);
    }

    #[test]
    fn test_split_at_end() {
        let mut data = vec![1, 2, 3];
        let (left, right) = split_at_safe(&mut data, 3);
        assert_eq!(left, &[1, 2, 3]);
        assert!(right.is_empty());
    }

    #[test]
    #[should_panic(expected = "mid out of bounds")]
    fn test_split_at_out_of_bounds() {
        let mut data = vec![1, 2, 3];
        split_at_safe(&mut data, 4);
    }

    // --- SimpleVec ---

    #[test]
    fn test_simple_vec_new() {
        let v = SimpleVec::<i32>::new();
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
        assert_eq!(v.capacity(), 0);
    }

    #[test]
    fn test_simple_vec_push_pop() {
        let mut v = SimpleVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn test_simple_vec_get() {
        let mut v = SimpleVec::new();
        v.push(10);
        v.push(20);
        v.push(30);
        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v.get(1), Some(&20));
        assert_eq!(v.get(2), Some(&30));
        assert_eq!(v.get(3), None);
    }

    #[test]
    fn test_simple_vec_with_capacity() {
        let v = SimpleVec::<i32>::with_capacity(10);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 10);
    }

    #[test]
    fn test_simple_vec_grow() {
        let mut v = SimpleVec::new();
        for i in 0..100 {
            v.push(i);
        }
        assert_eq!(v.len(), 100);
        assert!(v.capacity() >= 100);
        assert_eq!(v.get(99), Some(&99));
    }

    #[test]
    fn test_simple_vec_strings() {
        let mut v = SimpleVec::new();
        v.push("hello".to_string());
        v.push("world".to_string());
        assert_eq!(v.get(0), Some(&"hello".to_string()));
        assert_eq!(v.pop(), Some("world".to_string()));
    }

    // --- Unsafe Traits ---

    #[test]
    fn test_zeroed_i32() {
        let arr: [i32; 3] = create_zeroed_array();
        assert_eq!(arr, [0, 0, 0]);
    }

    #[test]
    fn test_zeroed_f64() {
        let arr: [f64; 2] = create_zeroed_array();
        assert_eq!(arr, [0.0, 0.0]);
    }

    #[test]
    fn test_zeroed_bool() {
        let arr: [bool; 4] = create_zeroed_array();
        assert_eq!(arr, [false, false, false, false]);
    }

    // --- FFI ---

    #[test]
    fn test_cstring() {
        let s = CString::new("hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_c_strlen() {
        let s = CString::new("hello world");
        unsafe {
            assert_eq!(c_strlen(s.as_ptr()), 11);
        }
    }

    #[test]
    fn test_c_strlen_empty() {
        let s = CString::new("");
        unsafe {
            assert_eq!(c_strlen(s.as_ptr()), 0);
        }
    }

    #[test]
    fn test_c_strcmp_equal() {
        let a = CString::new("hello");
        let b = CString::new("hello");
        unsafe {
            assert_eq!(c_strcmp(a.as_ptr(), b.as_ptr()), 0);
        }
    }

    #[test]
    fn test_c_strcmp_not_equal() {
        let a = CString::new("abc");
        let b = CString::new("abd");
        unsafe {
            assert!(c_strcmp(a.as_ptr(), b.as_ptr()) < 0);
            assert!(c_strcmp(b.as_ptr(), a.as_ptr()) > 0);
        }
    }

    #[test]
    fn test_c_memcpy() {
        let src = [1u8, 2, 3, 4, 5];
        let mut dst = [0u8; 5];
        unsafe {
            c_memcpy(dst.as_mut_ptr(), src.as_ptr(), 5);
        }
        assert_eq!(dst, [1, 2, 3, 4, 5]);
    }

    // --- Safe Abstraction ---

    #[test]
    fn test_shared_counter() {
        let counter = SharedCounter::new();
        assert_eq!(counter.get(), 0);
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    // --- Transmute ---

    #[test]
    fn test_float_to_bits_roundtrip() {
        let f: f32 = 3.14;
        let bits = float_to_bits(f);
        let back = bits_to_float(bits);
        assert_eq!(f, back);
    }

    #[test]
    fn test_float_bits_one() {
        // 1.0f32 = 0x3F800000
        assert_eq!(float_to_bits(1.0), 0x3F800000);
    }

    #[test]
    fn test_is_nan_check() {
        assert!(is_nan_check(f32::NAN));
        assert!(!is_nan_check(1.0));
        assert!(!is_nan_check(0.0));
        assert!(!is_nan_check(f32::INFINITY));
    }

    #[test]
    fn test_inspect_float_bits() {
        let (sign, _, _) = inspect_float_bits(1.0);
        assert!(!sign);
        let (sign, _, _) = inspect_float_bits(-1.0);
        assert!(sign);
    }

    // --- NonEmptyVec ---

    #[test]
    fn test_non_empty_vec_new() {
        let nev = NonEmptyVec::new(42);
        assert_eq!(nev.len(), 1);
        assert_eq!(nev.first(), &42);
        assert_eq!(nev.last(), &42);
    }

    #[test]
    fn test_non_empty_vec_from_vec() {
        let nev = NonEmptyVec::from_vec(vec![1, 2, 3]).unwrap();
        assert_eq!(nev.len(), 3);
        assert_eq!(nev.first(), &1);
        assert_eq!(nev.last(), &3);
    }

    #[test]
    fn test_non_empty_vec_from_empty() {
        let nev = NonEmptyVec::<i32>::from_vec(vec![]);
        assert!(nev.is_none());
    }

    #[test]
    fn test_non_empty_vec_push() {
        let mut nev = NonEmptyVec::new(1);
        nev.push(2);
        nev.push(3);
        assert_eq!(nev.len(), 3);
        assert_eq!(nev.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_non_empty_vec_to_vec() {
        let nev = NonEmptyVec::from_vec(vec![10, 20, 30]).unwrap();
        assert_eq!(nev.to_vec(), vec![10, 20, 30]);
    }
}
