# css/css-multicol/multicol-span-all-rule-002.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-rule-002.html"
}
```

## style[0]

```css

section {
  display: inline-block;
  width: 100px;
  height: 400px;
  border: 2px solid purple;
  columns: 2;
  column-gap: 10px;
  column-rule: 10px solid orange;
}

.span {
  column-span: all;
  block-size: 20px;
}

.p {
  block-size: 80px;
  background-color: lightgreen;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
