# css/css-images/radial-gradient-line-height-relative-units-001.html

```json
{
  "format_version": 3,
  "file": "css/css-images/radial-gradient-line-height-relative-units-001.html"
}
```

## style[0]

```css

.ref {
  position: absolute;
  z-index: 0;
  width: 200px;
  height: 200px;
  background-image: radial-gradient(25px at 100px 50px, green 100%, transparent);
}
.box {
  position: absolute;
  z-index: -1;
  width: 200px;
  font-size: 50px;
  line-height: 2;
  aspect-ratio: 1;
  background-image: radial-gradient(25px at 1lh 50px, red 100%, green);
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
