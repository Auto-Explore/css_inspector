# css/css-inline/text-box-trim/text-box-trim-text-emphasis-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-text-emphasis-001.html"
}
```

## style[0]

```css

.spacer {
  height: 40px;
  background: lightgray;
}
.target {
  font: 40px/2 Ahem;
  text-box-trim: trim-both;
  text-box-edge: text;
}
em {
  text-emphasis: dot;
  text-emphasis-position: over;
}
em + em {
  text-emphasis-position: under;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
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
