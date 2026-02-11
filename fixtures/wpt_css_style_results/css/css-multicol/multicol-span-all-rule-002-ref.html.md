# css/css-multicol/multicol-span-all-rule-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-rule-002-ref.html"
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
  column-gap: 20px;
  column-rule: 20px solid transparent;
}

.gap {
  background-color: orange;
  block-size: 10px;
}

.p {
  block-size: 195px;
  background-color: lightgreen;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
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
