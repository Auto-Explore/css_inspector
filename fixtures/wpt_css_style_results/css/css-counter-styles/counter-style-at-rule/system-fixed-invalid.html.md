# css/css-counter-styles/counter-style-at-rule/system-fixed-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/system-fixed-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: fixed;
    suffix: ":";
  }
  @counter-style b {
    system: fixed invalid;
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
