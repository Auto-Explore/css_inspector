# css/css-text/white-space/trailing-other-space-separators-break-spaces-007.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/trailing-other-space-separators-break-spaces-007.html"
}
```

## style[0]

```css

div {
  white-space: break-spaces;
  font-size: 10px;
  line-height: 1;
  width: 2em;
}
section {
  font-family: Ahem;
  float: left;
  margin: 0 1em;
  color: blue;
}
.ref {
  color: orange;
}
.ref div {
  white-space: pre;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
