# css/css-transforms/change-scale-wide-range.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/change-scale-wide-range.html"
}
```

## style[0]

```css

#target {
  will-change: transform;
  transform-origin: 0 0;
  width: 200px;
  height: 200px;
  background: green;
  border: 10px solid blue;
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
