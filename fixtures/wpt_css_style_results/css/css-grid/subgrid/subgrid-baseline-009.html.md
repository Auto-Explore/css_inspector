# css/css-grid/subgrid/subgrid-baseline-009.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-baseline-009.html"
}
```

## style[0]

```css

.item {
  writing-mode: vertical-rl;
  block-size: 80px;
  box-sizing: border-box;
  border: solid 5px hotpink;
  line-height: 0;
  margin-block-start: 10px;
  margin-block-end: 15px;
}
.small {
  width: 20px;
  height: 20px;
  border: solid 5px cyan;
}
.first {
  justify-self: baseline;
}
.last {
  justify-self: last baseline;
}
span {
  width: 20px;
  height: 20px;
  box-sizing: border-box;
  border: solid 5px orange;
  display: inline-block;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
