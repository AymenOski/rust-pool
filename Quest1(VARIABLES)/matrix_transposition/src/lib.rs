//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file

#[derive(Debug , PartialEq , Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));


pub fn transpose(m: Matrix) -> Matrix {
    return Matrix((m.0.0,m.1.0),(m.0.1,m.1.1));
}

/*
    * ═══════════════════════════════════════════════════════════════════════════════════
    A matrix can be represented as a 2D array, it is used in linear algebra, computer graphics, machine learning, physics simulations, and many other fields. Transposing a matrix is a common operation that involves flipping the matrix over its diagonal, which can be useful for various applications such as solving systems of linear equations, changing the orientation of data, and optimizing certain algorithms.
    * Q1 : what do i mean by linear equations?
    - A linear equation is an algebraic equation in which each term is either a constant or the product of a constant and a single variable. The general form of a linear equation in two variables (x and y) is ax + by = c, where a, b, and c are constants. Linear equations can be represented in matrix form, and solving systems of linear equations often involves operations on matrices, such as transposition, to find the solution, which is a set of values for the variables that satisfy all the equations in the system.
    * Q2 : what do i mean by changing the orientation of data?
    - Changing the orientation of data refers to the process of rearranging the structure of data, such as converting rows to columns or vice versa. This is often done to make the data more suitable for analysis, visualization, or to meet the requirements of a specific algorithm. For example, in a dataset where each row represents a different observation and each column represents a different variable, transposing the data would switch the rows and columns, allowing you to analyze the data from a different perspective or to perform operations that require a specific orientation.
    * Q3 : what do i mean by optimizing certain algorithms?
    - Optimizing certain algorithms refers to the process of improving the efficiency and performance of algorithms, often by reducing their time complexity or space complexity. In the context of matrix operations, transposing a matrix can sometimes lead to more efficient computations, especially in algorithms that involve matrix multiplication or solving linear systems. For example, some algorithms may perform better when the data is stored in a specific orientation (row-major or column-major order), and transposing the matrix can help achieve that orientation, leading to faster computations and reduced memory usage.
    * Q4 : what do i mean by row-major or column-major order?
    - Row-major order and column-major order are two ways of storing multi-dimensional arrays (like matrices) in linear memory. In row-major order, the elements of each row of the array are stored in contiguous memory locations, meaning that the entire first row is stored first, followed by the second row, and so on. In column-major order, the elements of each column are stored in contiguous memory locations, meaning that the entire first column is stored first, followed by the second column, and so on. The choice between row-major and column-major order can affect the performance of algorithms that access the data, as it can influence cache locality and the efficiency of memory access patterns.
    * Q5 : what do i mean by cache locality?
    - Cache locality refers to the principle that programs tend to access a relatively small portion of their memory at any given time. There are two types of cache locality: temporal locality and spatial locality. Temporal locality means that if a program accesses a particular memory location, it is likely to access the same location again in the near future. Spatial locality means that if a program accesses a particular memory location, it is likely to access nearby memory locations soon. Optimizing for cache locality can improve the performance of algorithms, as it can reduce the number of cache misses and improve the efficiency of memory access. For example, when processing a matrix stored in row-major order, accessing elements in a row-wise manner can take advantage of spatial locality, while accessing elements in a column-wise manner may lead to more cache misses.
    * Q6 : how the cache works in this context?
    - In the context of matrix operations, the cache is a small, fast memory that stores frequently accessed data to improve performance.
    - low level wise    - When a program accesses a memory location, the CPU checks if the data is in the cache, the CPU cache is devided into cache lines, which are small blocks of memory (usually 64 bytes). there are different levels of cache (L1, L2, L3).
        why L1 is faster than L2 and L3? because L1 is the smallest and closest to the CPU, while L2 and L3 are larger and farther away, which means that accessing data from L1 is faster than accessing data from L2 or L3.
        but why they don't have more size; how is size related to speed? because larger caches can store more data, but they also have longer access times due to the increased complexity of managing a larger cache and the physical distance from the CPU. As a result, there is a trade-off between cache size and speed, and the design of the cache hierarchy aims to balance these factors to optimize overall performance.
        sizes : L1 cache is typically around 32KB to 64KB, L2 cache is around 256KB to 512KB, and L3 cache can be several megabytes (e.g., 8MB or more).
    * ═══════════════════════════════════════════════════════════════════════════════════
*/

/*
* ═══════════════════════════════════════════════════════════════════════════════════
    * Q1 : what is the derive?
    - The derive attribute in Rust is a convenient way to automatically implement certain traits for a struct or enum. When you use #[derive(TraitName)], the Rust compiler generates the necessary code to implement the specified trait for your type. In this case, #[derive(Debug, PartialEq, Eq)] automatically implements the Debug trait (which allows for formatting the type using {:?}), the PartialEq trait (which allows for equality comparisons using ==), and the Eq trait (which indicates that the type has a total equivalence relation). This can save you time and effort, as you don't have to manually write out the implementations for these traits.
    * Q2 : derive is a macro, right?
    - yes, it is a procedural macro that generates code at compile time based on the specified traits.
    * Q3 : macro is a way to write code that writes other code, right?
    - yes, macros in Rust allow you to write code that generates other code, which can be useful for reducing boilerplate and creating more flexible and reusable code.    
* ═══════════════════════════════════════════════════════════════════════════════════
*/
