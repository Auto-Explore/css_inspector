# css/css-conditional/container-queries/canvas-as-container-003.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/canvas-as-container-003.html"
}
```

## style[0]

```css

  @supports (container-type: size) {
    canvas:focus-within {
      background-color: green;
    }
    canvas {
      display: block;
      width: 100px;
      height: 100px;
      container-type: size;
    }
    #target { display: none; }
    @container (width = 100px) {
      #target { display: block; }
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
