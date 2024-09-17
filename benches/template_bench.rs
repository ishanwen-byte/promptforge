use criterion::{black_box, criterion_group, criterion_main, Criterion};
use handlebars::Handlebars;
use std::collections::HashMap;

fn benchmark_complex_handlebars_template(c: &mut Criterion) {
    let mut handlebars = Handlebars::new();

    let template = r#"
    System: 
    You are an AI assistant. You should respond to user queries in a detailed and helpful manner, drawing on a wide range of knowledge from various domains, such as technology, history, and science. You should also be polite, clear, and concise in your responses.

    Instructions:
    {{#if instructions}}
    - {{instructions}}
    {{/if}}

    User: 
    {{user_message}}

    {{#if user_context}}
    Context:
    {{user_context}}
    {{/if}}

    Assistant:
    {{#if previous_assistant_message}}
    - Prior Assistant's Message: {{previous_assistant_message}}
    {{/if}}

    {{#if user_followup_message}}
    User Follow-up: 
    {{user_followup_message}}
    {{/if}}

    Response:
    {{#if assistant_replies}}
    {{#each assistant_replies}}
    - {{this}}
    {{/each}}
    {{else}}
    {{{assistant_generated_response}}}
    {{/if}}

    Notes: 
    {{#if additional_notes}}
    - {{additional_notes}}
    {{/if}}
    "#;

    handlebars
        .register_template_string("complex_template", template)
        .unwrap();

    let mut data = HashMap::new();
    data.insert(
        "instructions",
        "Please provide a detailed response based on the user’s request.",
    );
    data.insert(
        "user_message",
        "Can you explain how quantum computing works?",
    );
    data.insert(
        "user_context",
        "The user is a computer science student interested in quantum mechanics.",
    );
    data.insert("previous_assistant_message", "Sure! Quantum computing is a type of computation that takes advantage of quantum phenomena.");
    data.insert(
        "user_followup_message",
        "Could you explain the difference between classical and quantum computers in detail?",
    );
    data.insert("assistant_generated_response", "In classical computers, data is processed using bits, which represent either a 0 or a 1. Quantum computers, on the other hand, use quantum bits, or qubits, which can represent 0, 1, or both simultaneously thanks to superposition... This allows quantum computers to perform many calculations at once, leading to potentially exponential speed-ups for certain problems. For example, in cryptography, quantum computers could factor large numbers exponentially faster than classical computers, threatening current encryption methods. Another example is optimization problems, where quantum computers could explore many possible solutions simultaneously, drastically reducing the time to find the optimal solution.");

    c.bench_function("render complex handlebars template", |b| {
        b.iter(|| {
            let result = handlebars.render("complex_template", &black_box(data.clone()));
            black_box(result)
        });
    });
}

criterion_group!(benches, benchmark_complex_handlebars_template);
criterion_main!(benches);
