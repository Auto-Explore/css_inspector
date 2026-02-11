# css/css-masking/mask-image/mask-image-data-url-image.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-data-url-image.html"
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
  background: blue;
}
#front {
  position: absolute;
  box-sizing: border-box;
  width: 200px;
  height: 200px;
  border: 40px solid blue;
  background: green;
  mask-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVQImWNgYGDwAQAAUQBNt+pgmgAAAABJRU5ErkJggg==); /* 1x1 black with 30% transparency */
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
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
