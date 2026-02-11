# css/css-writing-modes/tcy-white-space-processing-003.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/tcy-white-space-processing-003.html"
}
```

## style[0]

```css

.v-rl {
  writing-mode: vertical-rl;
  width: 200px;
}
#test span {
  -webkit-text-combine: horizontal; /*testing the layout text-combine, not it's support in general*/
  text-combine-upright: all;
}
#test2 span {
  -webkit-text-combine: horizontal; /*testing the layout text-combine, not it's support in general*/
  text-combine-upright: all;
  white-space: pre;
}
#ref span {
  display: inline-block;
  height: 1em;
}
#test { color: blue; }
#test2 { color: brown; }
#ref { color: orange; }
.v-rl > div {
  display: inline-block;
  border: solid;
  margin: 0 5px;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-combine”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-combine”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
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
