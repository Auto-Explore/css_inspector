# css/css-borders/tentative/border-shape/border-shape-overflow-solid-background-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-overflow-solid-background-ref.html"
}
```

## style[0]

```css

  .br-target {
    overflow-y: scroll;
    will-change: transform;
    width: 100px;
    height: 100px;
    border-radius: 50%;
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
