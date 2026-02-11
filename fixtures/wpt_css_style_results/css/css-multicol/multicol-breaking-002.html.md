# css/css-multicol/multicol-breaking-002.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-breaking-002.html"
}
```

## style[0]

```css


.outer {
  height: 100px;
  column-fill: auto;
  width: 800px;
  column-count: 4;
  column-rule: 4px solid blue;
  column-gap: 16px;
  background: rgba(0, 0, 255, 0.3);
}

.inner {
  column-count: 2;
  column-rule: 2px solid fuchsia;
  column-gap: 16px;
  background: rgba(255, 0, 255, 0.3);
  font: 16px/1.25 sans-serif;
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
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
