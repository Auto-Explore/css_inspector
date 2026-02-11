# css/css-conditional/container-queries/crashtests/chrome-bug-372358471-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/chrome-bug-372358471-crash.html"
}
```

## style[0]

```css

  body {
    &::first-letter { color: green; }
    &::before { content: "."; }
  }
  canvas {
    float: left;
    container-type: inline-size;
    &::first-line {
      color: pink;
    }
  }
  :first-of-type {}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
