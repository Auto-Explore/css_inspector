# css/css-flexbox/max-width-violation.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/max-width-violation.html"
}
```

## style[0]

```css

.columns {
  display: flex;
  width: 800px;
}

.columns > div {
  background: #aaa;
}

.column1 {
  width: 800px;
  overflow: auto;
  max-width: 150px;
}

.column2 {
  flex: 0.8 0 0;
}

.red {
  position: absolute;
  width: 510px;
  background: red !important;
  height: 10px;
  z-index: -1;
}

.abspos {
  position: absolute;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
