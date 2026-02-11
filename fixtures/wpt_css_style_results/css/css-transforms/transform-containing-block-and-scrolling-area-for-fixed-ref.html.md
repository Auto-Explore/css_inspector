# css/css-transforms/transform-containing-block-and-scrolling-area-for-fixed-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-containing-block-and-scrolling-area-for-fixed-ref.html"
}
```

## style[0]

```css

  html, body { margin: 0; padding: 0 }
  #transformed {
    margin-left: 10px;
    margin-top: 10px;
    width: 200px;
    height: 200px;
    background: grey;
  }

  #fixed {
    width: 50px;
    height: 50px;
    background: green;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
