# css/css-writing-modes/tcy-white-space-processing-002.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/tcy-white-space-processing-002.html"
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
  white-space: pre;
}
#test { color: blue; }
#ref { color: orange; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
