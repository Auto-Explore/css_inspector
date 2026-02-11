# css/css-backgrounds/background-repeat-space-6-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-space-6-ref.html"
}
```

## style[0]

```css

      .outer
      {
        border: 1px solid black;
        width: 192px;
        height: 106px;
        display: flex;
        align-content: space-between;
        flex-wrap: wrap;
      }
      .inner
      {
        height: 32px;
        width: 64px;
        background-image: url(support/aqua-yellow-32x32.png);
        background-repeat: no-repeat;
        background-size: 64px 32px;
      }
      .inner_gradient
      {
        height: 32px;
        width: 64px;
        background-image: linear-gradient(to top left, red, green);
        background-repeat: no-repeat;
        background-size: 64px 32px;
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
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
