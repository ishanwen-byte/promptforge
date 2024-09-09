# PromptForge

**PromptForge** is a Rust library designed for building, formatting, and managing prompts for AI agents. With support for both F-string-like and Mustache-style templating, PromptForge allows developers to create dynamic and customizable prompts for use with Large Language Models (LLMs) and various AI-driven applications.

## Key Features

- **Template Flexibility**: PromptForge provides two powerful templating engines: **FmtString**, which is inspired by Python's F-strings, and **Mustache**, a widely used logic-less templating system. These tools allow you to define templates that are flexible, expressive, and reusable across different AI tasks.
  
- **Dynamic Prompt Construction**: You can define placeholders in your templates, and dynamically insert variables at runtime to generate context-specific prompts for your AI models. This makes PromptForge a great tool for use cases like chatbot conversations, task automation, and AI content generation.

- **Compatibility with LLMs**: PromptForge is designed to help manage prompts for Large Language Models (LLMs) like OpenAI’s GPT models and other AI platforms. Whether you need simple text completion or complex interactive AI behavior, PromptForge gives you the power to generate prompts that align with your needs.

- **Error Handling**: The library provides robust error detection for malformed templates, including identifying mismatched or mixed formatting styles, invalid placeholders, and other common issues in prompt construction.

- **Extensibility**: Developers can easily extend PromptForge to support custom templating engines or additional placeholder validation strategies.

## Acknowledgments

PromptForge draws inspiration from the excellent work done in the [LangChain prompts library](https://github.com/langchain-ai/langchain/tree/master/libs/core/langchain_core/prompts). LangChain’s approach to managing prompts and integrating with LLMs served as a valuable reference in the design and development of PromptForge, especially in terms of structuring reusable, dynamic prompts for AI applications.

## Goals

PromptForge is aimed at simplifying the process of working with AI-driven systems, especially when it comes to generating and managing prompts. Here are some of the core goals:

- **Ease of Use**: Provide a simple yet flexible API for developers to define, manage, and format prompts for LLMs and AI agents.
- **Consistency**: Ensure that templates are handled consistently and efficiently, whether using FmtString or Mustache-style placeholders.
- **Error Resilience**: PromptForge aims to identify and gracefully handle errors such as mixed template formats, malformed variable definitions, and invalid placeholders.
- **Modularity**: Encourage reusability and scalability by making the library modular, allowing developers to customize it to meet their needs.

## Planned Features

- **Custom Output Parsers**: Support for defining how the output from AI systems should be parsed and validated.
- **Template Caching**: Improve performance for applications that reuse templates by caching compiled templates.
- **Asynchronous Support**: As AI systems often operate asynchronously, PromptForge will include support for building and processing prompts asynchronously to align with modern async Rust patterns.
- **Advanced Templating**: Extend the current functionality with support for more advanced Mustache features, such as loops and conditionals, as well as other templating systems.

## Installation

PromptForge will soon be available on [crates.io](https://crates.io), making installation as simple as adding the following to your `Cargo.toml`:

```toml
[dependencies]
promptforge = "0.1"
```

## Contribution

Contributions are welcome! If you're interested in contributing to PromptForge, please take a moment to review the following guidelines:

1. **Fork the repository**: Create a new branch for your work and submit a pull request when ready.
2. **Follow Rust idioms**: Ensure that your code is idiomatic and follows Rust’s best practices for error handling, concurrency, and performance.
3. **Testing**: Add unit and integration tests for any new features or bug fixes.
4. **Documentation**: Update the documentation to reflect your changes, ensuring the README, comments, and docstrings are clear.
5. **Code Reviews**: All submissions will go through a code review process. Make sure to address feedback promptly.

If you encounter any issues, feel free to open an issue on the repository with details about the bug or feature request. 

## License

PromptForge is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contact

For questions or discussions, feel free to reach out or submit an issue on the GitHub repository.
