# css/css-inline/text-box-trim/text-box-trim-half-leading-block-box-003.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-half-leading-block-box-003.html"
}
```

## style[0]

```css

.div-parent {
  outline: 1px solid orange;
  font: 20px/3 Ahem;
  text-box-trim: trim-both;
  text-box-edge: text;
}
span {
  border-block: solid red;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-trim”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-edge”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-block”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
