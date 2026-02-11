# css/css-rhythm/content-based-height-rounds-up-to-step-unit.html

```json
{
  "format_version": 3,
  "file": "css/css-rhythm/content-based-height-rounds-up-to-step-unit.html"
}
```

## style[0]

```css

.container {
  display: flow-root;
  width: 100px;
  background-color: green;
}
.block-step {
  width: min-content;
  block-step-size: 100px;
  color: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “block-step-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
