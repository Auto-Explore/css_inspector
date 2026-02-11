# css/css-shapes/shape-outside-invalid-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside-invalid-001.html"
}
```

## style[0]

```css

#shape{
    width: 300px;
    height: 300px;
    shape-outside: invalid(50px,50px,50px,50px);
}
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
