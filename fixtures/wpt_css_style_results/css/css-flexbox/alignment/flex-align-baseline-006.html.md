# css/css-flexbox/alignment/flex-align-baseline-006.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/alignment/flex-align-baseline-006.html"
}
```

## style[0]

```css

#target {
  display: flex;
  flex-direction: column;
  height: 100px;
  align-items: last baseline;
  width: 200px;
  border: solid 3px;
  position: relative;
  line-height: 0;
  writing-mode: vertical-rl;
}
span {
  display: inline-block;
  width: 1em;
  height: 1em;
  outline: solid cornflowerblue 3px;
  outline-offset: -3px;
}
#target > div {
  flex: none;
}
#target > :nth-child(1) {
  background: lime;
  margin-bottom: 20px;
  padding-bottom: 20px;
  font-size: 20px;
  writing-mode: horizontal-tb;
}
#target > :nth-child(2) {
  background: hotpink;
  font-size: 30px;
  writing-mode: horizontal-tb;
}
#target > :nth-child(3) {
  background: papayawhip;
  font-size: 10px;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “align-items”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
