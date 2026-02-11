# css/css-images/image-orientation/reference/image-orientation-mask-image-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-orientation/reference/image-orientation-mask-image-ref.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 50px;
  background: blue;
  mask-mode: luminance;
}
.orient { mask-image: url(../support/exif-orientation-2-ur-pre-rotated.jpg); }
.no-orient { mask-image: url(../support/exif-orientation-9-u.jpg); }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “mask-mode”.",
      "severity": "Error"
    },
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
