# css/css-transforms/transform-origin-006.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-origin-006.html"
}
```

## style[0]

```css

      body > div {
        width: 190px;
        height: 90px;
        padding: 5px;
        border: 1px solid black;
        margin: 5px;
        transform: rotate(45deg);
        transform-origin: 50% 50%;
        position: relative;
        right: 5px;
      }
      body > div > div {
        margin: -5px;
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
