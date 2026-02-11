# css/css-writing-modes/tcy-white-space-processing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/tcy-white-space-processing-001.html"
}
```

## style[0]

```css

.v-rl {
  writing-mode: vertical-rl;
  width: 200px;
}
span {
  -webkit-text-combine: horizontal; /*testing the layout text-combine, not it's support in general*/
  text-combine-upright: all;
  white-space: normal;
}
#test { color: blue; }
#ref { color: orange; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-combine”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
