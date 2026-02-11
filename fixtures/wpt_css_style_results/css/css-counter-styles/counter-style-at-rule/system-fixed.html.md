# css/css-counter-styles/counter-style-at-rule/system-fixed.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/system-fixed.html"
}
```

## style[0]

```css

  @counter-style a {
    system: fixed;
    symbols: \25F0  \25F1  \25F2  \25F3;
    suffix: ':';
  }
  @counter-style b {
    system: fixed -1;
    symbols: \25F4  \25F5  \25F6  \25F7;
    suffix: ':';
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
