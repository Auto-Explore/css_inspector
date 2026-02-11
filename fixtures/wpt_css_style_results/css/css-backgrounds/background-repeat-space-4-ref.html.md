# css/css-backgrounds/background-repeat-space-4-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-repeat-space-4-ref.html"
}
```

## style[0]

```css

      .outer
      {
        border: 1px solid black;
        width: 96px;
        height: 106px;
        display: flex;
        align-content: space-between;
        flex-wrap: wrap;
      }
      .inner
      {
        height: 32px;
        width: 32px;
        background-image: url(support/aqua-yellow-32x32.png);
      }
      .inner_gradient
      {
        height: 32px;
        width: 32px;
        background-size: 32px 32px;
        background-image: linear-gradient(to top left, red, green);
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
