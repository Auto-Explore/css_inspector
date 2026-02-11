# css/css-images/image-set/image-set-zero-resolution-rendering-2.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-set/image-set-zero-resolution-rendering-2.html"
}
```

## style[0]

```css

#test {
    background-image: url("/images/red.png");
    background-image: image-set(url("/images/red.png") calc(0dppx * -1));
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
