# css/css-highlight-api/painting/custom-highlight-painting-below-selection.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-below-selection.html"
}
```

## style[0]

```css

  ::highlight(foo) {
    background: cyan;
    color: blue;
  }
  ::selection {
    background: blue;
    color: cyan;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
