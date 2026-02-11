# css/css-grid/alignment/grid-align-baseline-fieldset-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-align-baseline-fieldset-001.html"
}
```

## style[0]

```css

.target {
  display: grid;
  width: min-content;
  grid-auto-flow: column;
  position: relative;
  line-height: 0;
}
.target > div {
  background: hotpink;
  font-size: 30px;
}
fieldset {
  padding: 10px;
  border: solid 10px;
  margin: 0;
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
