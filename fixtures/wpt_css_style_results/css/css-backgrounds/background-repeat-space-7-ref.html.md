# css/css-backgrounds/background-repeat-space-7-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-space-7-ref.html"
}
```

## style[0]

```css

      .outer
      {
        border: 1px solid black;
        width: 106px;
        height: 192px;
        display: flex;
        justify-content: space-between;
        flex-wrap: wrap;
      }
      .inner
      {
        height: 64px;
        width: 32px;
        background-image: url(support/aqua-yellow-32x32.png);
        background-repeat: no-repeat;
        background-size: 32px 64px;
      }
      .inner_gradient
      {
        height: 64px;
        width: 32px;
        background-image: linear-gradient(to top left, red, green);
        background-repeat: no-repeat;
        background-size: 32px 64px;
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
