# css/css-break/table/table-row-paint-vrl-rtl.html

```json
{
  "format_version": 3,
  "file": "css/css-break/table/table-row-paint-vrl-rtl.html"
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
  background: repeating-linear-gradient(to left, orange, orange 30px, dodgerblue 30px, dodgerblue 60px);
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
