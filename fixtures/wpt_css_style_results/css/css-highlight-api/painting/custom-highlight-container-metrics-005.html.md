# css/css-highlight-api/painting/custom-highlight-container-metrics-005.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-container-metrics-005.html"
}
```

## style[0]

```css

    .wrapper {
      container: wrapper / size;
      width: 200px;
      height: 100px;
    }
    @container (width > 100px) {
      .wrapper::highlight(highlight1) {
        text-underline-offset: 2cqw;
        text-decoration-line: underline;
        text-decoration-color: green;
        text-decoration-thickness: 4cqh;
      }
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “container”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
