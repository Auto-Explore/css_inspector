# css/css-counter-styles/cssom/cssom-range-setter-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/cssom/cssom-range-setter-invalid.html"
}
```

## style[0]

```css

@counter-style foo {
  system: cyclic;
  symbols: A B C;
  range: 1 2;
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
