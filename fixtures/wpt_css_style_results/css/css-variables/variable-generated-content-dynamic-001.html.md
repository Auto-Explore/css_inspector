# css/css-variables/variable-generated-content-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-generated-content-dynamic-001.html"
}
```

## style[0]

```css

div::before {
  --my-attr: attr(data-foo);
  content: var(--my-attr);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
