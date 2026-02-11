# css/css-images/image-set/image-set-zero-resolution-rendering.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-zero-resolution-rendering.html"
}
```

## style[0]

```css

#test {
    background-image: url("/images/red.png");
    background-image: image-set(url("/images/red.png") 0x);
    width: 100px;
    height: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
