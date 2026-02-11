# css/css-masking/mask-image/mask-image-clip-exclude.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-clip-exclude.html"
}
```

## style[0]

```css

div {
  display: inline-block;
  width: 100px;
  height: 100px;
  padding: 10px;
  background: linear-gradient(green, green) border-box;
  mask: linear-gradient(red, red) content-box exclude, linear-gradient(red, red);
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
      "message": "Unknown property “mask”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
