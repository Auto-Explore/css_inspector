# css/css-borders/tentative/border-shape/border-shape-shadow-negative-spread.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-shadow-negative-spread.html"
}
```

## style[0]

```css

body {
    margin: 0;
}
#target {
    width: 100px;
    height: 100px;
    border-shape: inset(0px);
    background: blue;
    box-shadow: 20px 20px 0 -10px red;
    margin: 40px;
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
