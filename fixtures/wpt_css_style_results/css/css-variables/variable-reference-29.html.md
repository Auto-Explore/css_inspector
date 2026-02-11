# css/css-variables/variable-reference-29.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-29.html"
}
```

## style[0]

```css

p {
  color: red;
  --a: green;
  color: var(--a /* unclosed comment
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unclosed comment.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
