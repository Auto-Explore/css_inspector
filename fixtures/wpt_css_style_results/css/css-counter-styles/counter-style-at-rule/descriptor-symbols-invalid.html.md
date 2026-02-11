# css/css-counter-styles/counter-style-at-rule/descriptor-symbols-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-symbols-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: fixed;
    symbols: a b c;
    symbols: 0 1 2;
  }
  @counter-style b {
    system: additive;
    additive-symbols: 3 c, 2 b, 1 a;
    additive-symbols: 1 x, 2 y, 3 z;
    additive-symbols: x, y, z;
    additive-symbols: 1, 2, 3;
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
