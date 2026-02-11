# css/css-counter-styles/counter-style-at-rule/system-alphabetic-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/system-alphabetic-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: alphabetic;
    suffix: ":";
  }
  @counter-style b {
    system: alphabetic;
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
