# {{ function_name }}

{{ function_docstring.title }}
{% for part in function_docstring.body -%}
{%-if part.CodeSnippet -%}
```python
{{ part.CodeSnippet }}
```
{%-elif part.Text %}
{{ part.Text }}
{% endif %}
{% endfor-%}

{% if function_docstring.arguments %}
## Arguments:


| Name | Type | Description |
| --- | --- | --- |
{% for argument in function_arguments-%}
| {{ argument.name }} | {{ argument.type }} | {{ argument.description }} |
{% endfor %}
{% endif %}

{% if function_docstring.returns %}
## Returns:

{{ function_docstring.returns }}
{% endif %}

{% if function_docstring.raises %}
## Raises:

{{ function_docstring.raises }}
{% endif %}
