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

{% for argument in function_docstring.arguments-%}
- **{{argument.name}}:**
    - default: {{argument.default}}
    - description: {{argument.description}}
{% endfor-%}
{% endif %}
