# css/css-counter-styles/counter-style-at-rule/descriptor-range-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/descriptor-range-invalid.html"
}
```

## style[0]

```css

  @counter-style a {
    system: extends lower-alpha;
    range: 1 2;
    range: 1;
    range: 3 1;
    range: xx yy;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
