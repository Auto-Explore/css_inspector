# css/css-grid/alignment/grid-align-baseline-line-clamp-003.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-align-baseline-line-clamp-003.tentative.html"
}
```

## style[0]

```css

.target {
  display: grid;
  grid-auto-flow: column;
  inline-size: 100px;
  position: relative;
  line-height: 0;
  padding: 10px;
  writing-mode: vertical-lr;
}
.target > div {
  background: hotpink;
  font-size: 30px;
}
.line-clamp {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  border: solid 10px;
  font-size: 40px !important;
  overflow: hidden;
}
.line-clamp div {
  background: orange;
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
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-box-orient”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-line-clamp”.",
      "severity": "Error"
    },
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
