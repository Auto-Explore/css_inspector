# css/css-transforms/transform-compound-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-compound-001.html"
}
```

## style[0]

```css

      body {
        overflow: hidden;
      }
      div {
        transform-origin: top left;
      }
      body > div {
        position: relative;
        left: 200px;
        top: 0;
      }
      body > div > div {
        background-color: gold;
        width: 200px;
        height: 100px;
        transform: translate(100px) scale(2) rotate(90deg) skewX(15deg);
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
