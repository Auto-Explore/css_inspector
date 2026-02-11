# css/css-counter-styles/counter-style-at-rule/system-extends.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/system-extends.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends upper-roman;
    prefix: "Chapter ";
    range: 1 5;
  }
  @counter-style b {
    system: extends a;
    prefix: "Section ";
    range: 1 6;
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
