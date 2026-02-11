# css/css-paint-api/paint-function-arguments-var.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/paint-function-arguments-var.https.html"
}
```

## style[0]

```css

:root {
  --color1:rgb(50, 100, 150);
  --color2:rgb(150, 100, 50);
  --length1:50px;
}

.container {
  width: 200px;
  height: 200px;
}

#canvas-box-1 {
  background-image: paint(box, var(--color1), var(--length1));
}

#canvas-box-2 {
  background-image: paint(box, var(--color2), 100px);
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
