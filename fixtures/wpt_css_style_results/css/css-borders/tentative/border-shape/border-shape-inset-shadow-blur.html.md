# css/css-borders/tentative/border-shape/border-shape-inset-shadow-blur.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-inset-shadow-blur.html"
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
    background: white;
    border: 10px solid black;
    border-shape: rect(0, 0, 100%, 100%);
    box-shadow: inset 0 0 20px 0px red;
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
