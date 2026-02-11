# css/css-highlight-api/painting/custom-highlight-painting-iframe-004.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-iframe-004.html"
}
```

## style[0]

```css

  ::highlight(foo) {
    color: green;
    background-color: greenyellow;
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

## style[1]

```css

      ::highlight(foo) {
        color: blue;
        background-color: cyan;
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
