# css/css-backgrounds/background-repeat-space-2-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-space-2-ref.html"
}
```

## style[0]

```css

      .outer {
        width: 48px;
        height: 48px;
        border: 1px solid black;
        background-image: url(support/aqua-yellow-32x32.png);
        background-repeat: no-repeat;
        background-position: 5px 5px;
      }
      .outer_gradient {
        width: 48px;
        height: 48px;
        border: 1px solid black;
        background-size: 32px 32px;
        background-image: linear-gradient(to top left, red, green);
        background-repeat: no-repeat;
        background-position: 5px 5px;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
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
