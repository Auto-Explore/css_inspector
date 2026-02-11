# css/css-backgrounds/background-repeat-space-1a.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-space-1a.html"
}
```

## style[0]

```css

      .outer {
        width: 106px;
        height: 106px;
        border: 1px solid black;
        background-image: url(support/aqua-yellow-32x32.png);
        background-repeat: space;
      }
      .outer_gradient {
        width: 106px;
        height: 106px;
        border: 1px solid black;
        background-size: 32px 32px;
        background-image: linear-gradient(to top left, red, green);
        background-repeat: space;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
