# css/css-backgrounds/background-repeat-round-2.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-round-2.html"
}
```

## style[0]

```css

    .outer {
      width: 72px;
      height: 72px;
      border: 1px solid black;
      background-image: url(support/aqua-yellow-32x32.png);
      background-repeat: round no-repeat;
    }
    .outer_gradient {
      width: 72px;
      height: 72px;
      border: 1px solid black;
      background-size: 32px 36px;
      background-image: linear-gradient(to top left, red, green);
      background-repeat: round no-repeat;
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
