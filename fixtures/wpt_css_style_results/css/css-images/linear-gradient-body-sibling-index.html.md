# css/css-images/linear-gradient-body-sibling-index.html

```json
{
  "format_version": 3,
  "file": "css/css-images/linear-gradient-body-sibling-index.html"
}
```

## style[0]

```css

  body {
    background-position: top left;
    background-repeat: no-repeat;
    background-size: 100px 100px;
    background-image: linear-gradient(blue calc(20px * sibling-index()), yellow);
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
