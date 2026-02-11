# css/css-images/css-image-fallbacks-and-annotations005.html

```json
{
  "format_version": 3,
  "file": "css/css-images/css-image-fallbacks-and-annotations005.html"
}
```

## style[0]

```css

        .square{
            width: 200px;
            height: 200px;
            background-color: red;
            background-image: image(rgba(0,0,255,0.5)), url("support/1x1-green.png");
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
