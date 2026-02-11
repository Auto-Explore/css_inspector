# css/css-borders/tentative/border-shape/border-shape-shadow-semitransparent.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-shadow-semitransparent.html"
}
```

## style[0]

```css

body {
    margin: 0;
    background: white;
}
#target {
    width: 190px;
    height: 190px;
    border-shape: xywh(0% 0% 100% 100%);
    position: relative;
    background: green;
    box-shadow: 10px 20px 0 10px rgba(0, 0, 255, 0.5);
    border: 10px solid black;
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
