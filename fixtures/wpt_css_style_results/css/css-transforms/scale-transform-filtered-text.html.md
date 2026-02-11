# css/css-transforms/scale-transform-filtered-text.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/scale-transform-filtered-text.html"
}
```

## style[0]

```css

  #container {
    transform: scale(0.5);
    background-color: lightblue;
    font-size: 80px;
  }
  #content {
    filter: blur(0px);
  }
  #animated {
    will-change: transform;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
