// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

// Rust 非常擅长与 C/C++和其他静态编译语言共享外部函数接口（FFI），甚至可以在代码内部进行链接！
// 它通过`extern`块实现这一点，就像下面的代码那样。

// `extern`关键字后面的短字符串表示外部导入的函数将遵循哪种应用二进制接口（ABI）。
// 在这个练习中，使用了“Rust”，同时还存在其他变体，比如“C”表示标准 C ABI，“stdcall”表示 Windows ABI。

// 外部导入的函数在`extern`块中声明，用分号来标记签名的结束而不是花括号。
// 一些属性可以应用于这些函数声明以修改链接行为，例如`#[link_name = ".."]`用于修改实际的符号名称。

// 如果你想将你的符号导出到链接环境中，`extern`关键字也可以在具有相同 ABI 字符串注释的函数定义之前标记。
// Rust 函数的默认 ABI 实际上是“Rust”，所以如果你想链接纯 Rust 函数，整个`extern`术语可以省略。

// Rust 默认会像 C++一样对符号进行修饰。为了抑制这种行为并使这些函数可以通过名称寻址，可以应用`#[no_mangle]`属性。

// 在这个练习中，你的任务是使测试用例能够调用模块`Foo`中的`my_demo_function`。
// `my_demo_function_alias`是`my_demo_function`的别名，所以测试用例中的两行代码应该调用同一个函数。

// 除了添加两行属性之外，你不应该修改任何现有代码。

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    pub extern "C" fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
