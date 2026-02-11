# css/css-borders/tentative/border-shape/border-shape-two-shapes-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-two-shapes-shadow.html"
}
```

## style[0]

```css

body {
    margin: 0;
}
#target {
    width: 200px;
    height: 200px;
    border-shape: circle(50%) circle(25%);
    position: relative;
    background: green;
    border-color: yellow;
    box-shadow: 10px 10px 0 5px blue, 0 0 0 5px purple inset;
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
