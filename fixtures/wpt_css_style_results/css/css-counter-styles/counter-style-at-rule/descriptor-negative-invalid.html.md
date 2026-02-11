# css/css-counter-styles/counter-style-at-rule/descriptor-negative-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-negative-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends decimal;
    negative: '!';
    negative: 0;
    negative: ~;
    negative: '(' 'x' ')';
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
