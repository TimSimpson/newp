import copy

from newp import templates

TEST_TEMPLATE = {
    "README.md": """
# Example

This will use the project name, which is "{{name}}", in a variety of ways, like:

    camel_case: {{camel_case_name}}
    kebab_case: {{kebab_case_name}}
    pascal_case: {{pascal_case_name}}
    scream_case: {{scream_case_name}}
    snake_case: {{snake_case_name}}

Wow!""",
    "{{name}}.txt": "{{description}}",
}


def test_evaluate() -> None:
    options = templates.Options("test", "TestProject", "/tmp", "Hello there!")

    expected = {
        "README.md": """
# Example

This will use the project name, which is "TestProject", in a variety of ways, like:

    camel_case: testProject
    kebab_case: test-project
    pascal_case: TestProject
    scream_case: TEST_PROJECT
    snake_case: test_project

Wow!""",
        "TestProject.txt": "Hello there!",
    }
    actual = templates._evaluate(options, copy.deepcopy(TEST_TEMPLATE))
    assert expected == actual
