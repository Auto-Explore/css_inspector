# css/css-paint-api/background-image-tiled.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/background-image-tiled.https.html"
}
```

## style[0]

```css

    div {
        display: inline-block;
        width: 100px;
        height: 100px;
    }

    #one {
        background:
            paint(ellipse) top left/50% 50% repeat-x,
            paint(ellipse) bottom left/100% 50% no-repeat;
    }

    #two {
        background:
            paint(ellipse) top left/50% 20% repeat-y,
            paint(ellipse) center right/50% 50% no-repeat;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
