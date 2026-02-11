# css/css-borders/tentative/border-shape/border-shape-overflow-solid-background.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-overflow-solid-background.html"
}
```

## style[0]

```css

  .bs-target {
    overflow-y: scroll;
    will-change: transform;
    width: 100px;
    height: 100px;
    border-shape: circle(50% at 50% 50%);
    background: green;
  }
  .forcescroll {
    width: 25px;
    height: 400px;
    background: linear-gradient(pink, blue);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “border-shape”.",
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
