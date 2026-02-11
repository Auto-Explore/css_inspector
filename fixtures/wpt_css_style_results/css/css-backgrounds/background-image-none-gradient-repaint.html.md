# css/css-backgrounds/background-image-none-gradient-repaint.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-image-none-gradient-repaint.html"
}
```

## style[0]

```css

  #box {
    width: 150px;
    height: 150px;
  }
  .red {
    background: none, linear-gradient(to right, red, red);
  }
  .green {
    background: none, linear-gradient(to right, green, green);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
