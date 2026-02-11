# css/css-transforms/transform-fixed-bg-006.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-fixed-bg-006.html"
}
```

## style[0]

```css

      body {
        margin: 0;
        transform: rotate(90deg);
        transform-origin: 100px 100px;
        overflow: hidden;
      }
      div {
        background: url(support/transform-triangle-down.svg) fixed;
        width: 100px;
        height: 100px;
        position: relative;
        left: 50px;
        top: 50px;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
