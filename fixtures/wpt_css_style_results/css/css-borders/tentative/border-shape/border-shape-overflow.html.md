# css/css-borders/tentative/border-shape/border-shape-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-overflow.html"
}
```

## style[0]

```css

#target {
    width: 100px;
    height: 100px;
    border-shape: polygon(50% 0, 100% 50%, 50% 100%, 0 50%);
    border-radius: 10px;
    position: relative;
    overflow: hidden;
}
#inner {
    background: green;
    position: absolute;
    inset: 0;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “border-shape”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
