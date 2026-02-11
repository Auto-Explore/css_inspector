# css/css-inline/text-box-trim/text-box-trim-float-start-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-float-start-001.html"
}
```

## style[0]

```css

.float {
  float: left;
  width: 100px;
  height: 50px;
  background: yellow;
}
.text {
  margin-top: 100px;
  font: 50px/3 Ahem;
  text-box-trim: trim-start;
  text-box-edge: text;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “text-box-trim”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-edge”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
