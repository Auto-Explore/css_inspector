# css/css-inline/text-box-trim/text-box-trim-multicol-003.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-multicol-003.html"
}
```

## style[0]

```css

.spacer {
  background: lightgray;
  block-size: 100px;
}
.multicol {
  column-count: 3;
  column-fill: auto;
  border: 1px solid;
  height: 290px;
  width: 20ch;
  font-family: Ahem;
  font-size: 40px;
  line-height: 2;
  orphans: 1;
  widows: 1;
}
.trim {
  text-box-trim: trim-both;
  text-box-edge: text;
  box-decoration-break: clone;
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
