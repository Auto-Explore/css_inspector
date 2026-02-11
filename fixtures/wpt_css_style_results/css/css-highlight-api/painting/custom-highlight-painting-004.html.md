# css/css-highlight-api/painting/custom-highlight-painting-004.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-004.html"
}
```

## style[0]

```css

  ::highlight(foo) {
    color:blue;
    background-color:yellow;
  }
  ::highlight(bar) {
    background-color:orange;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
