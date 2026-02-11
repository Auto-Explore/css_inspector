# css/css-variables/variable-reference-28.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-28.html"
}
```

## style[0]

```css

p {
  color: red;
  --a: green;
  color: var(--a, var(--b
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
