# css/css-counter-styles/cssom/cssom-fallback-setter-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/cssom/cssom-fallback-setter-invalid.html"
}
```

## style[0]

```css

@counter-style foo {
  system: fixed;
  symbols: A B;
  fallback: lower-roman;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
