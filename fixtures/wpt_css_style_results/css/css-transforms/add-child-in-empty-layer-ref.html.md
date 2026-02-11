# css/css-transforms/add-child-in-empty-layer-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/add-child-in-empty-layer-ref.html"
}
```

## style[0]

```css

.empty-layer {
  transform: rotateY(30deg) rotateX(-30deg);
  width: 100px;
  height: 100px;
}
.inserted {
  width: 50px;
  height: 50px;
  background-color: red;
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
