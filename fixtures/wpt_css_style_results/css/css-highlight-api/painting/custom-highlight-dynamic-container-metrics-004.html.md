# css/css-highlight-api/painting/custom-highlight-dynamic-container-metrics-004.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-dynamic-container-metrics-004.html"
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
