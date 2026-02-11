# css/css-highlight-api/painting/custom-highlight-painting-019.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-019.html"
}
```

## style[0]

```css

  body {
    text-decoration: 2px green underline;
  }
  ::highlight(foo) {
    color:blue;
    text-decoration: 2px blue underline;
  }
  ::highlight(bar) {
    text-decoration-line: underline;
    text-decoration-thickness: 2px;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
