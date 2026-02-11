# css/css-images/conic-gradient-line-height-relative-units-003.html

```json
{
  "format_version": 3,
  "file": "css/css-images/conic-gradient-line-height-relative-units-003.html"
}
```

## style[0]

```css

.base {
  position: absolute;
  width: 200px;
  height: 200px;
  background: green;
  z-index: -2;
}
.ref {
  position: absolute;
  width: 200px;
  height: 200px;
  mask-image: conic-gradient(from 45deg at 100px 50px, green 25%, transparent 0);
  background: green;
  z-index: 0;
}
.box {
  position: absolute;
  z-index: -1;
  width: 200px;
  font-size: 50px;
  line-height: 2;
  aspect-ratio: 1;
  mask-image: conic-gradient(from 45deg at 1lh 50px, green 25%, transparent 0);
  background: red;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
