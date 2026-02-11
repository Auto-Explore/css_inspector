# css/css-break/table/table-grid-paint-vrl-rtl-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/table/table-grid-paint-vrl-rtl-ref.html"
}
```

## style[0]

```css

body {
  writing-mode: vertical-rl;
  direction: rtl;
}
.multicol {
  inline-size: 400px;
  block-size: 100px;
  columns: 4;
  column-fill: auto;
  gap: 0;
}
.pattern {
  background: repeating-linear-gradient(to left, orange, orange 25px, dodgerblue 25px, dodgerblue 50px);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
