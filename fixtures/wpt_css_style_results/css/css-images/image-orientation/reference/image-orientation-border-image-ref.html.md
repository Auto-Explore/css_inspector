# css/css-images/image-orientation/reference/image-orientation-border-image-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-orientation/reference/image-orientation-border-image-ref.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 50px;
  border: 10px solid black;
}
.orient { border-image: url(../support/exif-orientation-2-ur-pre-rotated.jpg) 10; }
.no-orient { border-image: url(../support/exif-orientation-9-u.jpg) 10; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
