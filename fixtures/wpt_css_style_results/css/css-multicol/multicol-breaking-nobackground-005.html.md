# css/css-multicol/multicol-breaking-nobackground-005.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-breaking-nobackground-005.html"
}
```

## style[0]

```css


.outer {
  height: 200px;
  width: 800px;
  column-fill: balance;
  column-count: 3;
  column-gap: 16px;
  background: rgba(0, 0, 255, 0.3);
}

.inner {
  column-count: 2;
  column-fill: balance;
  column-rule: 2px solid fuchsia;
  column-gap: 16px;
  font: 16px/1.25 sans-serif;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
