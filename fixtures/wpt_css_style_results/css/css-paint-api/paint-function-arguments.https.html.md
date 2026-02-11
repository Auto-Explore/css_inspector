# css/css-paint-api/paint-function-arguments.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/paint-function-arguments.https.html"
}
```

## style[0]

```css

.container {
  width: 200px;
  height: 200px;
}

#canvas-box-1 {
  background-image: paint(box, rgb(50, 100, 150), 50px);
}

#canvas-box-2 {
  background-image: paint(box, rgb(150, 100, 50), 100px);
}

#background {
  background-color: yellow;
  display: inline-block;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
