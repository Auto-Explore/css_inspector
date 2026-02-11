# css/css-paint-api/background-image-alpha.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/background-image-alpha.https.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
}

#canvas-opaque {
  background-image: paint(opaque);
}

#canvas-nonopaque {
  background-image: paint(nonOpaque);
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
