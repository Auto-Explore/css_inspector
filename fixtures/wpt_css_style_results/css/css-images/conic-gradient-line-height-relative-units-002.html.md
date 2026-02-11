# css/css-images/conic-gradient-line-height-relative-units-002.html

```json
{
  "format_version": 3,
  "file": "css/css-images/conic-gradient-line-height-relative-units-002.html"
}
```

## style[0]

```css

.ref {
  position: absolute;
  z-index: 0;
  width: 200px;
  height: 200px;
  background-image: conic-gradient(from 45deg at 50px 100px, green 25%, transparent 0);
}
.box {
  position: absolute;
  z-index: -1;
  width: 200px;
  font-size: 50px;
  line-height: 2;
  aspect-ratio: 1;
  background-image: conic-gradient(from 45deg at 50px 1lh, red 25%, green 0);
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
