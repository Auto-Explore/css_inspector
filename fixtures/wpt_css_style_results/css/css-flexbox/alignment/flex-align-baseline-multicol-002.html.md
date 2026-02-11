# css/css-flexbox/alignment/flex-align-baseline-multicol-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/alignment/flex-align-baseline-multicol-002.html"
}
```

## style[0]

```css

.target {
  display: flex;
  inline-size: 200px;
  position: relative;
  line-height: 0;
  margin: 40px;
  padding: 10px;
  writing-mode: vertical-rl;
}
.target > div {
  background: hotpink;
  font-size: 20px;
}
.multicol {
  columns: 3;
  column-fill: auto;
  padding: 10px;
  border: solid 10px;
}
.multicol > div {
  break-inside: avoid;
  break-before: column;
  break-after: column;
}
span {
  display: inline-block;
  width: 1em;
  height: 1em;
  outline: solid cyan 3px;
  outline-offset: -3px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
