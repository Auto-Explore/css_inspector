# css/css-break/transform-006.html

```json
{
  "format_version": 3,
  "file": "css/css-break/transform-006.html"
}
```

## style[0]

```css

  .transformed {
      transform: rotate(25deg);
      transform-origin: bottom right;
      background: hotpink;
  }
  .child {
      margin-left: auto;
      width: 50px;
      background: lime;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
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
