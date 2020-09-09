from newp import naming


def test_name_to_components() -> None:
    assert [] == naming.name_to_components("")
    assert ["hi"] == naming.name_to_components("hi")
    assert ["23", "hi"] == naming.name_to_components("23hi")
    assert ["why", "hello"] == naming.name_to_components("WhyHello")
    assert ["why", "hello"] == naming.name_to_components("Why Hello")
    assert ["why", "hello"] == naming.name_to_components("Why    Hello")
    assert ["why", "hello"] == naming.name_to_components("why__hello")
    assert ["why", "hello", "32"] == naming.name_to_components("why__hello32")


def test_name_camel_case() -> None:
    assert "" == naming.camel_case("")
    assert "hi" == naming.camel_case("hi")
    assert "23Hi" == naming.camel_case("23hi")
    assert "whyHello" == naming.camel_case("WhyHello")
    assert "whyHello" == naming.camel_case("Why Hello")
    assert "whyHello" == naming.camel_case("Why    Hello")
    assert "whyHello" == naming.camel_case("why__hello")
    assert "whyHello32" == naming.camel_case("why__hello32")


def test_kebab_case() -> None:
    assert "" == naming.kebab_case("")
    assert "hi" == naming.kebab_case("hi")
    assert "23-hi" == naming.kebab_case("23hi")
    assert "why-hello" == naming.kebab_case("WhyHello")
    assert "why-hello" == naming.kebab_case("Why Hello")
    assert "why-hello" == naming.kebab_case("Why    Hello")
    assert "why-hello" == naming.kebab_case("why__hello")
    assert "why-hello-32" == naming.kebab_case("why__hello32")


def test_name_pascal_case() -> None:
    assert "" == naming.pascal_case("")
    assert "Hi" == naming.pascal_case("hi")
    assert "23Hi" == naming.pascal_case("23hi")
    assert "WhyHello" == naming.pascal_case("WhyHello")
    assert "WhyHello" == naming.pascal_case("Why Hello")
    assert "WhyHello" == naming.pascal_case("Why    Hello")
    assert "WhyHello" == naming.pascal_case("why__hello")
    assert "WhyHello32" == naming.pascal_case("why__hello32")


def test_name_scream_case() -> None:
    assert "" == naming.scream_case("")
    assert "HI" == naming.scream_case("hi")
    assert "23_HI" == naming.scream_case("23hi")
    assert "WHY_HELLO" == naming.scream_case("WhyHello")
    assert "WHY_HELLO" == naming.scream_case("Why Hello")
    assert "WHY_HELLO" == naming.scream_case("Why    Hello")
    assert "WHY_HELLO" == naming.scream_case("why__hello")
    assert "WHY_HELLO_32" == naming.scream_case("why__hello32")


def test_snake_case() -> None:
    assert "" == naming.snake_case("")
    assert "hi" == naming.snake_case("hi")
    assert "23_hi" == naming.snake_case("23hi")
    assert "why_hello" == naming.snake_case("WhyHello")
    assert "why_hello" == naming.snake_case("Why Hello")
    assert "why_hello" == naming.snake_case("Why    Hello")
    assert "why_hello" == naming.snake_case("why__hello")
    assert "why_hello_32" == naming.snake_case("why__hello32")
