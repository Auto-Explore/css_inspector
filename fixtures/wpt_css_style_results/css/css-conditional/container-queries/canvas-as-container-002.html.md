# css/css-conditional/container-queries/canvas-as-container-002.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/canvas-as-container-002.html"
}
```

## style[0]

```css

  @supports (container-type: size) {
    canvas:focus-within {
      border: 50px solid green;
    }
    canvas {
      display: block;
      position: absolute;
      width: 100px;
      height: 100px;
      box-sizing: border-box;
      container-type: size;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
