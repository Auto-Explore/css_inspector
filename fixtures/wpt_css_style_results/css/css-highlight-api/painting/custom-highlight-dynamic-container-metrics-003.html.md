# css/css-highlight-api/painting/custom-highlight-dynamic-container-metrics-003.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-dynamic-container-metrics-003.html"
}
```

## style[0]

```css

    .wrapper {
      container: wrapper / size;
      width: 400px;
      height: 200px;
    }
    .resize {
      width: 200px;
      height: 100px;
    }
    @container (width < 300px) {
      ::highlight(highlight1) {
        text-underline-offset: 2cqw;
        text-decoration-line: underline;
        text-decoration-color: green;
        text-decoration-thickness: 4cqh;
      }
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “container”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
