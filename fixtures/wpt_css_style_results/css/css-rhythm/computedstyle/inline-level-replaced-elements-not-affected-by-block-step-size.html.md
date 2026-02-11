# css/css-rhythm/computedstyle/inline-level-replaced-elements-not-affected-by-block-step-size.html

```json
{
  "format_version": 3,
  "file": "css/css-rhythm/computedstyle/inline-level-replaced-elements-not-affected-by-block-step-size.html"
}
```

## style[0]

```css

.container {
  display: inline flow-root;
  width: 100px;
  font-size: 0px;
}
.block-step {
  block-step-size: 100px;
  visibility: hidden;
}

iframe {
  border: 0;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “display”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “block-step-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
