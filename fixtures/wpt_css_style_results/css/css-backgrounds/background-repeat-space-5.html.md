# css/css-backgrounds/background-repeat-space-5.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-space-5.html"
}
```

## style[0]

```css

      .outer {
        width: 106px;
        height: 96px;
        border: 1px solid black;
        background-image: url(support/aqua-yellow-32x32.png);
        background-repeat: space repeat;
      }
      .outer_gradient {
        width: 106px;
        height: 96px;
        border: 1px solid black;
        background-size: 32px 32px;
        background-image: linear-gradient(to top left, red, green);
        background-repeat: space repeat;
      }
    
```

```json
{
  "errors": 4,
  "messages": [
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
