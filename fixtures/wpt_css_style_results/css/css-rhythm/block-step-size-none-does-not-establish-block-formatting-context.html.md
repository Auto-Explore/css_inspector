# css/css-rhythm/block-step-size-none-does-not-establish-block-formatting-context.html

```json
{
  "format_version": 3,
  "file": "css/css-rhythm/block-step-size-none-does-not-establish-block-formatting-context.html"
}
```

## style[0]

```css

div {
    width: 100px;
    height: 100px;
    background-color: green;
}
.floating {
    position: relative;
    z-index: -1;
    float: left;
    background-color: red;
}
.block-step-size {
    block-step-size: none;
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
