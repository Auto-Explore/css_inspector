# css/css-transforms/transform-fixed-bg-004.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-fixed-bg-004.html"
}
```

## style[0]

```css

      body {
        margin: 0;
        height: 5000px;
        overflow: hidden;
      }
      div {
        background: url(support/transform-triangle-down.svg) fixed;
        width: 100px;
        height: 100px;
        transform: translate(50px, 150px) rotate(90deg);
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
