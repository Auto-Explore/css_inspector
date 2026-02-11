# css/css-grid/alignment/grid-align-baseline-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-align-baseline-004.html"
}
```

## style[0]

```css

#target {
  display: grid;
  grid-template: 50px 50px 50px / 50px 50px;
  grid-auto-flow: column;
  align-items: last baseline;
  width: 200px;
  border: solid 3px;
  position: relative;
  line-height: 0;
  font-size: 20px;
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
      "message": "Invalid value for property “align-items”.",
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
