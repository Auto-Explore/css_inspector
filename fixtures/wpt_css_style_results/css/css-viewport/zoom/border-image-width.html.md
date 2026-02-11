# css/css-viewport/zoom/border-image-width.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/border-image-width.html"
}
```

## style[0]

```css

.box {
  width: 60px;
  height: 60px;
  border-image: url("data:image/svg+xml,%3csvg width='100' height='100' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100' height='100' fill='red'/%3e%3c/svg%3e") 30;
  border-image-width: 8px;
  margin: 10px;
}
.zoom {
  zoom: 2;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
