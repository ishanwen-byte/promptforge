use promptforge::{FewShotTemplate, Formattable, Template};
use std::collections::HashMap;
use std::path::Path;

#[tokio::test]
async fn test_few_shot_template_with_prefix_suffix_and_examples() {
    let toml_path = Path::new("tests/data/few_shot_template.toml");

    let few_shot_template: FewShotTemplate<Template> =
        FewShotTemplate::from_toml_file(toml_path).await.unwrap();

    let mut variables = HashMap::new();
    variables.insert("topic", "Science");
    variables.insert("question1", "What is the speed of light?");
    variables.insert("answer1", "Approximately 299,792 kilometers per second.");
    variables.insert("question2", "What is the gravitational constant?");
    variables.insert("answer2", "Approximately 6.674×10^-11 N·(m/kg)^2.");

    let formatted_output = few_shot_template.format(&variables).unwrap();

    let expected_output = "\
This is the prefix. Topic: Science
---
Q: What is the speed of light?
A: Approximately 299,792 kilometers per second.
---
Q: What is the gravitational constant?
A: Approximately 6.674×10^-11 N·(m/kg)^2.
---
This is the suffix. Remember to think about Science.";

    assert_eq!(formatted_output, expected_output);
}
