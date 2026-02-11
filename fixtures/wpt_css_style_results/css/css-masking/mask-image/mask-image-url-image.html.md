# css/css-masking/mask-image/mask-image-url-image.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-url-image.html"
}
```

## style[0]

```css

#back {
  position: absolute;
  box-sizing: border-box;
  width: 200px;
  height: 200px;
  border: 60px solid green;
  background: red;
}
#front {
  position: absolute;
  box-sizing: border-box;
  width: 200px;
  height: 200px;
  border: 40px solid red;
  background: green;
  mask-image: url(support/image.svg);
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
