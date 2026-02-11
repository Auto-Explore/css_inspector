# css/css-rhythm/replaced-elements/block-level-svg-margins-affected-by-block-step-size.html

```json
{
  "format_version": 3,
  "file": "css/css-rhythm/replaced-elements/block-level-svg-margins-affected-by-block-step-size.html"
}
```

## style[0]

```css

.container {
  display: flow-root;
  background-color: green;
  width: 100px;
}
.block-step {
  display: block;
  block-step-size: 100px;
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
