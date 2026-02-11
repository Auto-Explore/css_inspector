# css/css-view-transitions/scoped/transform-clip-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/transform-clip-ref.html"
}
```

## style[0]

```css

  #clipper {
    background-color: hotpink;
    overflow: clip;
    height: 100px;
    width: 100px;
    border-radius: 50px;
    position: absolute;
    top: 20px;
    left: 20px;
  }

.item {
  position: relative;
  top: 25px;
  height: 50px;
  background-color: forestgreen;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
