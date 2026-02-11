# css/css-rhythm/replaced-elements/block-level-img-margins-affected-by-block-step-size.html

```json
{
  "format_version": 3,
  "file": "css/css-rhythm/replaced-elements/block-level-img-margins-affected-by-block-step-size.html"
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
  block-step-size: 100px;
  display: block;
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
