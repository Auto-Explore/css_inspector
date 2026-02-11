# css/css-grid/grid-lanes/tentative/alignment/column-justify-items-end-justify-self-start-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/column-justify-items-end-justify-self-start-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    position: relative;
    flow-tolerance: 0;
    grid-template-columns: repeat(3, 100px);
    width: 300px;
    justify-items: end;
}

.fifty-width {
    width: 50px;
    background-color: lightblue;
}
```

```json
{
  "errors": 2,
  "messages": [
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
