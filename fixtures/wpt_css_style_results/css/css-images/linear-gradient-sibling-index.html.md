# css/css-images/linear-gradient-sibling-index.html

```json
{
  "format_version": 3,
  "file": "css/css-images/linear-gradient-sibling-index.html"
}
```

## style[0]

```css

  .grad {
    width: 100px;
    height: 100px;
    background: linear-gradient(blue calc(10px * sibling-index()), yellow);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
