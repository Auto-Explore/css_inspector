# css/css-align/blocks/align-content-block-display-coverage.html

```json
{
  "format_version": 3,
  "file": "css/css-align/blocks/align-content-block-display-coverage.html"
}
```

## style[0]

```css

@import "/fonts/ahem.css";
body {
  font: 10px/1 Ahem;
  margin: 0;
}
.target {
  height: 50px;
  align-content: unsafe center;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Invalid value for property “align-content”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
