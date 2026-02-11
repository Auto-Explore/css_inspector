# css/css-transforms/transform-origin-003.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-origin-003.html"
}
```

## style[0]

```css

      div {
        width: 200px;
        height: 100px;
        border: 1px solid black;
        transform: rotate(45deg);
        transform-origin: 101px 51px;
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
