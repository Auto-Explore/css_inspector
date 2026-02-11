# css/css-page/page-orientation-on-portrait-003-notref.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-orientation-on-portrait-003-notref.html"
}
```

## style[0]

```css

@page {
  margin: 0;
  size: 200px 300px;
}
@page rotated {
  page-orientation: rotate-right;
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
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “page-orientation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
