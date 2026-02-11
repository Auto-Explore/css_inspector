# css/css-counter-styles/counter-style-at-rule/descriptor-fallback-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-fallback-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends upper-alpha;
    fallback: decimal-leading-zero;
    fallback: decimal cjk-decimal;
    fallback: "*";
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
