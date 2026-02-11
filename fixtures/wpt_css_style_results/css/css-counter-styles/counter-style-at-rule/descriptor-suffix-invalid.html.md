# css/css-counter-styles/counter-style-at-rule/descriptor-suffix-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-suffix-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends decimal;
    suffix: ',';
    suffix: *;
    suffix: 0;
    suffix: '$' '$';
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
