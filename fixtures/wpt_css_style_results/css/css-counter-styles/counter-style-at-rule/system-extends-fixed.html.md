# css/css-counter-styles/counter-style-at-rule/system-extends-fixed.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/system-extends-fixed.html"
}
```

## style[0]

```css

  @counter-style a {
      system: fixed 3;
      symbols: "Y" "E" "S";
  }
  @counter-style b {
      system: extends a;
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
