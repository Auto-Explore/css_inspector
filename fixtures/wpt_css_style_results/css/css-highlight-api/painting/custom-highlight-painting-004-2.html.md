# css/css-highlight-api/painting/custom-highlight-painting-004-2.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-004-2.html"
}
```

## style[0]

```css

  ::highlight(foo) {
    color:blue;
    background-color:yellow;
  }
  ::highlight(bar) {
    color:orange;
  }
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
