# css/css-images/image-rendering-border-image-notref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/image-rendering-border-image-notref.html"
}
```

## style[0]

```css

div {
  width: 200px;
  height: 150px;
  background-color: black;
  border: solid 4px;
  border-image: url(support/small-border.png) 4 / 16px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
