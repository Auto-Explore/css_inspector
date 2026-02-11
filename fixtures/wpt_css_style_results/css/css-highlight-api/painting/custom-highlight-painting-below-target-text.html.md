# css/css-highlight-api/painting/custom-highlight-painting-below-target-text.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-below-target-text.html"
}
```

## style[0]

```css

  ::highlight(foo) {
    color:limegreen;
    background-color:yellow;
  }
  ::target-text {
    background-color:orange;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
