# css/css-backgrounds/background-repeat-round-1d.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-round-1d.html"
}
```

## style[0]

```css

      .outer {
        width: 72px;
        height: 72px;
        border: 1px solid black;
        background-size: 32px 36px;
        background-image: url(support/aqua-yellow-32x32.png);
        background-repeat: round repeat;
      }
      .outer_gradient {
        width: 72px;
        height: 72px;
        border: 1px solid black;
        background-size: 32px 36px;
        background-image: linear-gradient(to top left, red, green);
        background-repeat: round repeat;
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
