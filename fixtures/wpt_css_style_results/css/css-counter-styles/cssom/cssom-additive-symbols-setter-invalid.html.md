# css/css-counter-styles/cssom/cssom-additive-symbols-setter-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/cssom/cssom-additive-symbols-setter-invalid.html"
}
```

## style[0]

```css

@counter-style foo {
  system: additive;
  additive-symbols: 2 C, 1 B, 0 A;
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
