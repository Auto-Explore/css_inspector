# css/css-images/radial-gradient-line-height-relative-units-004.html

```json
{
  "format_version": 3,
  "file": "css/css-images/radial-gradient-line-height-relative-units-004.html"
}
```

## style[0]

```css

.base {
  position: absolute;
  z-index: -2;
  width: 200px;
  height: 200px;
  background: green;
}
.ref {
  position: absolute;
  z-index: 0;
  width: 200px;
  height: 200px;
  background-image: radial-gradient(25px at 50px 100px, green 100%, transparent);
}
.box {
  position: absolute;
  z-index: -1;
  width: 200px;
  font-size: 50px;
  line-height: 2;
  aspect-ratio: 1;
  mask-image: radial-gradient(25px at 50px 1lh, red 100%, transparent);
  background: red;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
