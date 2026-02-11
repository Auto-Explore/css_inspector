# css/css-break/flexbox/single-line-column-flex-fragmentation-069d-print.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/single-line-column-flex-fragmentation-069d-print.html"
}
```

## style[0]

```css

@page { size: 5in 3in; margin: 0.5in; }
body {
  margin: 0;
  font-size: 0.25in;
}
.flexbox {
  display: flex;
  flex-direction: column;
  border: 0.25in solid black;
}
.flexbox > div {
  border: 4px solid purple;
}
.flexbox > .nested {
  border: 4px solid gold;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
