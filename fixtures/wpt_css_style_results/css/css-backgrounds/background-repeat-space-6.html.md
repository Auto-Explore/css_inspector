# css/css-backgrounds/background-repeat-space-6.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-space-6.html"
}
```

## style[0]

```css

      .outer {
        width: 192px;
        height: 106px;
        border: 1px solid black;
        background-size: 60px 32px;
        background-image: url(support/aqua-yellow-32x32.png);
        background-repeat: round space;
      }
      .outer_gradient {
        width: 192px;
        height: 106px;
        border: 1px solid black;
        background-size: 60px 32px;
        background-image: linear-gradient(to top left, red, green);
        background-repeat: round space;
      }
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
