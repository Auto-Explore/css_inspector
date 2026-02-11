# css/css-grid/grid-lanes/tentative/alignment/row-align-items-end-align-self-start-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/row-align-items-end-align-self-start-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    background: gray;
    flow-tolerance: 0;
    grid-template-rows: repeat(3, 100px);
    width: 300px;
    align-items: end;
}

.fifty-width {
    width: 50px;
    background-color: lightblue;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “flow-tolerance”.",
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
