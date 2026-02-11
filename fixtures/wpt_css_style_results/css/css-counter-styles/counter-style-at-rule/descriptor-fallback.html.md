# css/css-counter-styles/counter-style-at-rule/descriptor-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-fallback.html"
}
```

## style[0]

```css

  @counter-style a {
    system: fixed;
    symbols: a b c;
    fallback: b;
  }
  @counter-style b {
    system: fixed 4;
    symbols: d e f;
    fallback: a;
  }
  @counter-style c {
    system: fixed 7;
    symbols: g h i;
    fallback: a;
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
