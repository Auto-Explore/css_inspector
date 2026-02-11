# css/css-counter-styles/counter-style-at-rule/system-cyclic-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/system-cyclic-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: cyclic;
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
