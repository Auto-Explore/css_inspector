# css/css-backgrounds/background-repeat-round-3.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-round-3.html"
}
```

## style[0]

```css

      .outer {
        width: 72px;
        height: 72px;
        border: 1px solid black;
        background-image: url(support/aqua-yellow-32x32.png);
        background-size: 36px auto;
        background-repeat: no-repeat round;
      }
      .outer_gradient {
        width: 72px;
        height: 72px;
        border: 1px solid black;
        background-image: linear-gradient(to top left, red, green);
        background-size: 36px 32px;
        background-repeat: no-repeat round;
      }
    
```

```json
{
  "errors": 4,
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
      "message": "Invalid value for property “background-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
