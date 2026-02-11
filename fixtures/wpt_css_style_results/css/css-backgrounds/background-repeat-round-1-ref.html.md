# css/css-backgrounds/background-repeat-round-1-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-round-1-ref.html"
}
```

## style[0]

```css

      .outer {
        width: 72px;
        height: 72px;
        border: 1px solid black;
        background-image: url(support/aqua-yellow-32x32.png);
        background-size: 36px 36px;
        background-repeat: repeat;
      }
      .outer_gradient {
        width: 72px;
        height: 72px;
        border: 1px solid black;
        background-size: 36px 36px;
        background-image: linear-gradient(to top left, red, green);
        background-repeat: repeat;
      }
      
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
