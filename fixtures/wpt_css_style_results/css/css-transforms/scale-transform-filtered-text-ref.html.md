# css/css-transforms/scale-transform-filtered-text-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/scale-transform-filtered-text-ref.html"
}
```

## style[0]

```css

  #container {
    transform: scale(0.5);
    background-color: lightblue;
    font-size: 80px;
  }
  #animated, #content {
    will-change: transform;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
