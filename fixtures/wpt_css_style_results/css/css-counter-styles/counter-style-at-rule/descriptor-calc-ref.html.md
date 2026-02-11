# css/css-counter-styles/counter-style-at-rule/descriptor-calc-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-calc-ref.html"
}
```

## style[0]

```css

    @counter-style a {
      system: extends upper-roman;
      range: 1 6;
      pad: 4 '*';
    }
    @counter-style b {
      system: fixed 2;
      symbols: g h;
    }
    @counter-style c {
      system: additive;
      additive-symbols: 3 c, 2 b, 1 a;
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
