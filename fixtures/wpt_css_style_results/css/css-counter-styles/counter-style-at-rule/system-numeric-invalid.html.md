# css/css-counter-styles/counter-style-at-rule/system-numeric-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/system-numeric-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: numeric;
    suffix: ":";
  }
  @counter-style b {
    system: numeric;
    symbols: A;
    suffix: ":";
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
