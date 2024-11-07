# Capstone Project Brief: AI-Powered Personal Life Assistant

## Project Overview

Develop an AI-powered personal life assistant that enhances productivity, personal growth, and life organization. The project aims to leverage Rust for frontend development with WebAssembly, pushing Rust's presence in this space, while integrating AI capabilities for personalized assistance.

## Key Objectives

1. Enhance Rust's frontend presence by building a high-performance, type-safe application using Rust and WebAssembly.
2. Integrate AI capabilities for natural language processing and personalized recommendations.
3. Develop a cross-platform desktop application using Tauri.
4. Implement core features including smart task management, personalized recommendations, and goal tracking.
5. Ensure data privacy and security throughout the application.

## Technology Stack

- **Frontend**: Rust with WebAssembly, Yew Framework
- **Backend**: Elixir/Phoenix Framework OR Rust/Actix/Axum (TBD)
- **AI Integration**: Python AI models with communication bridge to Rust/Wasm via [Rust NLP library](#nlp-library-for-rust)
- **Desktop Packaging**: Tauri
- **Database**: PostgreSQL or SQLite with appropriate ORM

## Key Features

1. Task Management with natural language input
2. AI-driven personalized recommendations
3. Goal tracking and habit formation tools
4. Real-time updates and data synchronization
5. (Stretch Goal) User activity analytics and insights
   - Generate personalized reports on productivity trends
   - Provide AI-driven suggestions for improving habits and achieving goals
   - Visualize progress over time with interactive charts and graphs

## Project Scope

- Develop a fully functional desktop application
- Implement core features with AI integration
- Ensure cross-platform compatibility (Windows, macOS, Linux)
- Focus on performance optimization and security

## Learning Outcomes

- Enhance software architecture skills by designing a complex, multi-component system
- Strengthen proficiency in high-performance, type-safe programming using Rust and WebAssembly
- Develop expertise in cross-language system integration and communication protocols
- Gain experience in AI integration and application of machine learning models
- Improve skills in performance optimization and scalability across different system components
- Deepen understanding of secure coding practices and data protection in distributed systems
- Cultivate experience in cross-platform development and (stretch goal) deployment strategies

## Conclusion

This project aligns with the goal of enhancing Rust's competitive edge in frontend development while creating a valuable tool for personal productivity. It offers a unique opportunity to contribute to multiple tech ecosystems and tackle real-world challenges in software development.

### **NLP Library for Rust**

1. [**rust-bert**](https://github.com/guillaume-be/rust-bert)

   - Pros:
     - Specifically designed for NLP tasks, which aligns well with project's needs.
     - Provides high-level APIs for common NLP tasks like text classification and named entity recognition.
     - Well-documented with examples.
   - Cons:
     - Focused on transformer models, which might be overkill for simpler tasks.
