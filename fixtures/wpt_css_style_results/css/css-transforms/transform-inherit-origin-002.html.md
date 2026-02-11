# css/css-transforms/transform-inherit-origin-002.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-inherit-origin-002.html"
}
```

## style[0]

```css

      body {
        height: 50px;
        width: 50px;
        transform-origin: 50% 100%;
      }
      div {
        height: 100px;
        width: 100px;
        transform: rotate(180deg);
        transform-origin: inherit;
        background: blue;
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
