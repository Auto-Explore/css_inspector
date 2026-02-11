# css/css-viewport/zoom/mask-border-width.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/mask-border-width.html"
}
```

## style[0]

```css

    .box {
        width: 50px;
        height: 50px;
        background-color: red;
        mask-border-slice: 10;
        mask-border-source: linear-gradient(45deg, pink, blue, white, black, green);
        mask-border-width: 8px;
        margin: 10px;
    }

    .zoom {
        zoom: 2;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “mask-border-slice”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-source”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-border-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
