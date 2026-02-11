# css/css-page/page-orientation-on-portrait-002-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-orientation-on-portrait-002-print.html"
}
```

## style[0]

```css

@page {
  margin: 0;
  size: 200px 300px;
}
@page rotated {
  page-orientation: rotate-left;
}
body {
  margin: 0;
}
.filler {
  box-sizing: border-box;
  width: 200px;
  height: 300px;
  border: 10px solid black;
}
.specialborder {
  border-bottom-color: cyan;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “page-orientation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-*-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
