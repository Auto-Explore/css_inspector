# css/filter-effects/backdrop-filter-clip-radius-zoom-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-clip-radius-zoom-ref.html"
}
```

## style[0]

```css

body {
  zoom: 5;
}
div {
  width: 100px;
  height: 20px;
  background-color: black;
  border: 2px dashed green;
}
#rect1 {
  border-radius: 20px / 5px;
}
#rect2 {
  position: relative;
  top:5px;
  border-radius: 4px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
