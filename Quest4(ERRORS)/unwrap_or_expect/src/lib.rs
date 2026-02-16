//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub enum Security {
    Unknown, // Returns the server URL or panics.
    Message, // Returns the server URL or panics with the error message ERROR: program stops.
    Warning, // Returns the server URL or the message Not found: [MESSAGE], where [MESSAGE] represents the server's error message.
    NotFound, // NotFound: Returns the server URL or the message Not found: [MESSAGE], where [MESSAGE] represents the server's error message.
    UnexpectedUrl, // UnexpectedUrl: Returns the error message or panics with the error message being the server URL.
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message =>
            match server {
                Ok(url) => url.to_string(),
                Err(url) => panic!("ERROR: program stops"),
            }
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound =>
            match server {
                Ok(url) => url.to_string(),
                Err(url) => format!("Not found: {}", url),
            }
        _ =>
            match server {
                Ok(url) => panic!("{}", url),
                Err(url) => url.to_string(),
            }
    }
}

/*
    * Q & A: 
        * Q1: What is a macro?
        -A1: A macro is a powerful tool in Rust that allows you to write code that generates other code. It can be used to create functions, implement traits, or even generate entire modules. Macros are defined using the `macro_rules!` syntax and can take arguments to customize the generated code.
        * Q2: What is the difference between `unwrap` and `expect` in Rust?
        -A2: Both `unwrap` and `expect` are methods used to handle `Result` and `Option` types in Rust. The main difference is that `unwrap` will panic with a default error message if the value is an `Err` or `None`, while `expect` allows you to provide a custom error message that will be displayed when it panics. Using `expect` can make debugging easier by providing more context about the error.
        * Q3: How panic! works under the hood?
        -A3: The `panic!` macro in Rust is used to indicate that a program has encountered an unrecoverable error. When `panic!` is called, it unwinds the stack, which means it cleans up the current function's resources and then continues to unwind through the call stack until it reaches the main function. If the panic is not caught, the program will terminate with an error message. 
        * Q4: What is the philosophy behind Rust's error handling?
        -A4: Rust's error handling philosophy is centered around safety and explicitness. Instead of using exceptions like in other languages, Rust uses the `Result` and `Option` types to represent potential errors and the absence of values. This encourages developers to handle errors explicitly, making it less likely for bugs to go unnoticed. Rust also provides powerful tools like pattern matching and combinators to work with these types effectively, promoting a more robust and maintainable codebase.
        * Q5: What can we use Rust as a programming language for?
        -A5: Rust is a versatile programming language that can be used for a wide range of applications. It is particularly well-suited for systems programming, where performance and safety are critical. Rust is commonly used for developing operating systems, game engines, web browsers, and embedded systems. Additionally, Rust's focus on safety and concurrency makes it a great choice for building web servers, network services, and command-line tools. Its growing ecosystem and strong community support also make it an attractive option for general-purpose programming.
        & what do you mean by system programming?
        -System programming refers to the development of software that interacts closely with the underlying hardware and operating system. This includes tasks such as managing memory, handling input/output operations, and controlling hardware devices. System programming often involves writing low-level code that requires a deep understanding of the computer's architecture and operating system. Rust is particularly well-suited for system programming due to its focus on safety, performance, and concurrency, allowing developers to write efficient and reliable code while minimizing the risk of common programming errors like null pointer dereferences and data races.
        & what do you mean by embedded systems?
        -Embedded systems refer to specialized computing systems that are designed to perform specific tasks or functions within a larger system. ~ These systems are often resource-constrained, meaning they have limited processing power, memory, and storage compared to general-purpose computers. Examples of embedded systems include microcontrollers in household appliances, automotive control systems, medical devices, and IoT (Internet of Things) devices. Rust is a great choice for developing embedded systems because of its ability to provide low-level control over hardware while ensuring memory safety and preventing common programming errors. This makes it possible to create efficient and reliable embedded applications that can run on a wide range of devices.
        & front end development?
        -While Rust is primarily known for its strengths in systems programming, it can also be used for front-end development, particularly with the help of WebAssembly (Wasm). WebAssembly is a binary instruction format that allows code written in languages like Rust to run in web browsers at near-native speed. By compiling Rust code to WebAssembly, developers can create high-performance web applications that can interact with JavaScript and the DOM (Document Object Model). This makes Rust a viable option for front-end development, especially for performance-critical applications such as games, data visualization, and complex user interfaces. Additionally, frameworks like Yew and Seed provide tools and libraries to facilitate building front-end applications in Rust, making it easier for developers to leverage Rust's capabilities in the web development space.
        & why is Rust low level?
        -Rust is considered a low-level programming language because it provides direct access to memory and hardware, allowing developers to write code that can interact closely with the underlying system. Unlike high-level languages that abstract away details of memory management and hardware interaction, Rust gives developers control over these aspects while still providing safety guarantees. This means that Rust can be used for tasks that require fine-grained control over performance and resource management, such as systems programming, embedded development, and performance-critical applications. Rust's ability to manage memory safely without a garbage collector also contributes to its classification as a low-level language, as it allows developers to optimize their code for specific use cases while minimizing the risk of common programming errors.
        & what do u mean by abstracting away details of memory management and hardware interaction?
        -Abstracting away details of memory management and hardware interaction refers to the way high-level programming languages provide a layer of abstraction that hides the complexities of how memory is allocated, accessed, and managed, as well as how the program interacts with the underlying hardware. In high-level languages, developers typically do not need to worry about issues like memory leaks, pointer arithmetic, or direct hardware access, as these are handled by the language's runtime or garbage collector. This allows developers to focus more on the logic and functionality of their applications rather than dealing with low-level details. In contrast, low-level languages like Rust require developers to manage memory and hardware interactions explicitly, giving them more control but also requiring a deeper understanding of how the system works.
*/