# css/css-backgrounds/animations/background-color-animation-with-mask-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-with-mask-ref.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  -webkit-mask-image: url('../resources/stripes-100.png');
  -webkit-mask-size: 100px 100px;
  -webkit-mask-repeat: no-repeat;
  mask-image: url('../resources/stripes-100.png');
  mask-size: 100px 100px;
  mask-repeat: no-repeat;
  background-color:  rgb(0, 0, 150);
}
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “-webkit-mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-mask-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-mask-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
