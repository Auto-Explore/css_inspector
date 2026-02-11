# css/css-images/css-image-fallbacks-and-annotations.html

```json
{
  "format_version": 3,
  "file": "css/css-images/css-image-fallbacks-and-annotations.html"
}
```

## style[0]

```css

        .square{
            width: 200px;
            height: 200px;
            background-color: red;
            background: image("green.png", green);
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
