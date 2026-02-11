# css/css-images/css-image-fallbacks-and-annotations004.html

```json
{
  "format_version": 3,
  "file": "css/css-images/css-image-fallbacks-and-annotations004.html"
}
```

## style[0]

```css

        .square{
            width: 200px;
            height: 200px;
            background-color: red;
            background-image: image("1x1-green.svg", "1x1-green.png", "support/1x1-green.gif");
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
