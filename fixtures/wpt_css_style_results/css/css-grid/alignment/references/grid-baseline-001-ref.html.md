# css/css-grid/alignment/references/grid-baseline-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/references/grid-baseline-001-ref.html"
}
```

## style[0]

```css

body {
    margin: 0;
}
.inline-flexbox {
    display: inline-flex;
    background-color: lightgrey;
    margin-top: 5px;
}
.flexbox {
    display: flex;
    background-color: grey;
    margin-top: 10px;
}
.empty {
    border-style: solid;
    border-width: 5px 0px 10px;
    padding: 2px 0px 4px;
    margin: 10px 0px 20px;
}
.column {
    flex-flow: column;
}
.column-reverse {
    flex-flow: column-reverse;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
