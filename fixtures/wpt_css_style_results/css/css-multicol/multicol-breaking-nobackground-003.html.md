# css/css-multicol/multicol-breaking-nobackground-003.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-breaking-nobackground-003.html"
}
```

## style[0]

```css


.outer {
  height: 100px;
  column-fill: auto;
  width: 800px;
  column-count: 4;
  column-gap: 16px;
  background: rgba(0, 0, 255, 0.3);
}

.inner {
  column-count: 2;
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
