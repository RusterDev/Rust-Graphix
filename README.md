# RustGraphix Lib #

RustGraphix-Lib is a Rust library that provides a variety of features related to computer graphics and linear algebra. Designed to be intuitive and efficient, the library covers both basic and advanced operations in different domains, offering developers a powerful set of tools for working with graphics and matrices.

# Key Features:

Vector and Matrix Operations:

Comprehensive support for vector and matrix operations in two and three dimensions.
Functionality includes addition, subtraction, multiplication, transposition, and determinant calculation.
Graphics Manipulation:

Methods for creating, manipulating, and analyzing vectors and matrices in the context of computer graphics.
Transformation, normalization, and spatial analysis functionalities.
Three-Dimensional Matrices:

Library extension to include operations on three-dimensional matrices for advanced applications.
# Linear Algebra:

Implementation of fundamental linear algebra operations, including matrix multiplication, determinant calculation, and matrix inversion.
Color Handling:

Support for representing and manipulating colors in different formats.
Optimized Performance:

Optimized code to ensure efficient performance, especially in graphics-intensive calculations.
Ease of Integration:

Designed to be easily integrated into other Rust projects, providing a solid foundation for graphic development.
Detailed Documentation:

Comprehensive documentation to assist developers in understanding and effectively using the library.
RustGraphix-Lib is an ideal choice for developers seeking a robust and efficient solution for working with graphics and linear algebra in Rust projects. Whether creating games, visualizations, or scientific applications, this library provides the necessary tools to achieve high-quality visual and mathematical results.




---
----
---
---
---
---
---
---
---

---
---
---
---
---




## ```DEPENDENCIES ``` 
**nothing**
### 



## API Documentation

# METHODS AND CONSTRUCTORS # 

```http
  
```
| Method      | From          | Description                                           |
| :---------- | :------------ | :---------------------------------------------------- |
| `new`       | `Vec2`        | **CONSTRUCTOR**. Creates a new two-dimensional vector with specified x and y components. |
| `magnitude` | `Vec2`        | Computes the magnitude (length) of the vector using the Euclidean norm. |

**`vector::Op` Methods:**
| Method      | From          | Description                                           |
| :---------- | :------------ | :---------------------------------------------------- |
| `add`       | `vector::Op`  | **OP**. Returns the sum of two two-dimensional vectors without modifying the original vectors. |
| `sub`       | `vector::Op`  | **OP**. Returns the subtraction of two two-dimensional vectors without modifying the original vectors. |
| `dot`       | `vector::Op`  | **OP**. Returns the dot product between two two-dimensional vectors. |
| Method      | From              | Description                                           |
| :---------- | :---------------- | :---------------------------------------------------- |
| `distance`  | `vector::Magnitude` | **MAGNITUDE**. Computes the Euclidean distance between the current vector and another two-dimensional vector. |
| `normalize` | `vector::Magnitude` | **MAGNITUDE**. Computes and returns a normalized version of the vector, making it a unit vector. Divides each component of the vector by its magnitude. |
| Method      | From               | Description                                           |
| :---------- | :----------------- | :---------------------------------------------------- |
| `translate` | `vector::Transform` | **TRANSFORM**. Modifies the current vector by setting its x and y components to the x and y components of another two-dimensional vector. |
| Method      | From     | Description                                           |
| :---------- | :------- | :---------------------------------------------------- |
| `new`       | `Vector3` | **CONSTRUCTOR**. Creates a new three-dimensional vector with specified x, y, and z components. Computes and sets the magnitude of the vector. |

**`Op` Methods for `Vector3`:**

| Method      | From          | Description                                           |
| :---------- | :------------ | :---------------------------------------------------- |
| `add`       | `Op`          | **OP**. Returns the sum of two three-dimensional vectors. Modifies the current vector by adding the components of another vector. |
| `sub`       | `Op`          | **OP**. Returns the subtraction of two three-dimensional vectors. Computes a new vector with the difference between the current vector and another vector. |
| `dot`       | `Op`          | **OP**. Returns the dot product between two three-dimensional vectors. Computes the scalar product of the current vector and another vector. |

**`Transform` Methods for `Vector3`:**
| Method      | From           | Description                                           |
| :---------- | :------------- | :---------------------------------------------------- |
| `translate` | `Transform`    | **TRANSFORM**. Translates the vector in three-dimensional space by specified values along the x, y, and z axes. Modifies the current vector. |
| `normalize` | `Transform`    | **TRANSFORM**. Normalizes the vector, making it a unit vector. Divides each component of the vector by its magnitude, resulting in a vector with magnitude 1.0. Modifies the current vector. |
| Method      | From           | Description                                           |
| :---------- | :------------- | :---------------------------------------------------- |
| `Distance`  | `Analysis`     | **ANALYSIS**. Computes the Euclidean distance between the current vector and another three-dimensional vector. Returns a scalar value representing the distance. |
| Method                | From    | Description                                           |
| :-------------------- | :------ | :---------------------------------------------------- |
| `add`                 | `Op`    | **OP**. Returns the sum of two two-dimensional matrices. |
| `sub`                 | `Op`    | **OP**. Returns the subtraction of two two-dimensional matrices. |
| `scalar_multiply`     | `Op`    | **OP**. Returns the matrix multiplied by a scalar. |
| `multiply_matrices`   | `Op`    | **OP**. Returns the product of two two-dimensional matrices. |
| `transpose`           | `Op`    | **OP**. Returns the transpose of the matrix. |
| `determinant`         | `Op`    | **OP**. Returns the determinant of the matrix. |
| `cofactor_matrix`     | `Op`    | **OP**. Returns the cofactor matrix of the matrix. |
| `inverse_matrix`      | `Op`    | **OP**. Returns the inverse of the matrix if it exists, otherwise returns `None`. |
| `print_matrix`        | `Op`    | Prints the elements of the matrix. |
| Method                   | From    | Description                                           |
| :----------------------- | :------ | :---------------------------------------------------- |
| `add_3`                 | `Op3`   | **OP**. Returns the sum of two three-dimensional matrices. |
| `sub_3`                 | `Op3`   | **OP**. Returns the subtraction of two three-dimensional matrices. |
| `scalar_multiply_3`     | `Op3`   | **OP**. Returns the matrix multiplied by a scalar. |
| `multiply_matrices_3`   | `Op3`   | **OP**. Returns the product of two three-dimensional matrices. |
| `transpose_3`           | `Op3`   | **OP**. Returns the transpose of the matrix. |
| `determinant_3`         | `Op3`   | **OP**. Returns the determinant of the matrix. |
| `cofactor_matrix_3`     | `Op3`   | **OP**. Returns the cofactor matrix of the matrix. |
| `inverse_matrix_3`      | `Op3`   | **OP**. Returns the inverse of the matrix if it exists, otherwise returns `None`. |
| `print_matrix_3`        | `Op3`   | Prints the elements of the matrix. |
