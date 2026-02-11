# css/css-transforms/transform-compound-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-compound-ref.html"
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
      div.test {
        background-color: gold;
        width: 200px;
        height: 100px;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
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
