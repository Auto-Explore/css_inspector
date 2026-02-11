# css/css-flexbox/alignment/flex-align-baseline-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/alignment/flex-align-baseline-003.html"
}
```

## style[0]

```css

#target {
  display: flex;
  align-items: last baseline;
  width: 200px;
  border: solid 3px;
  position: relative;
  writing-mode: vertical-rl;
}
#target > :nth-child(1) {
  background: lime;
  margin-right: 20px;
  padding-right: 20px;
  font-size: 20px;
  line-height: 20px;
  align-self: first baseline;
  writing-mode: vertical-rl;
}
#target > :nth-child(2) {
  background: hotpink;
  font-size: 30px;
  line-height: 30px;
  writing-mode: vertical-lr;
}
#target > :nth-child(3) {
  background: papayawhip;
  font-size: 16px;
  line-height: 16px;
  writing-mode: vertical-lr;
}
#target > :nth-child(4) {
  background: orange;
  margin-left: 20px;
  padding-left: 20px;
  font-size: 20px;
  line-height: 20px;
  align-self: first baseline;
  writing-mode: vertical-lr;
}
#target > :nth-child(5) {
  background: cyan;
  font-size: 30px;
  line-height: 30px;
  writing-mode: vertical-rl;
}
#target > :nth-child(6) {
  background: papayawhip;
  font-size: 16px;
  line-height: 16px;
  writing-mode: vertical-rl;
}
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Invalid value for property “align-items”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
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
