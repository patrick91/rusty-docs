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

{% for argument in function_arguments-%}
- **{{argument.name}}:**
    {% if argument.type -%}
    - type: {{ argument.type }}
    {% endif -%}
    {% if argument.default -%}
    - default: {{ argument.default }}
    {% endif -%}
    - description: {{ argument.description }}
{% endfor-%}
{% endif %}
