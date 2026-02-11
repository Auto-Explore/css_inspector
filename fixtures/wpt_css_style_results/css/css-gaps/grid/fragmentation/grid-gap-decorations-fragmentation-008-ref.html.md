# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-008-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-008-ref.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multi-col {
    position: relative;
    height: 100px;
    width: 320px;
    columns: 3;
    column-fill: auto;
    column-gap: 10px;
    background: lightgray;
  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    /* Simulate row gaps with row tracks */
    grid-template-rows: repeat(2, 50px 20px) 50px 10px 50px;
    column-gap: 10px;
  }
  .grid-container>div {
    background-color: skyblue;
  }
  .column-gap {
    position: absolute;
    background: blue;
    height: 99px;
    width: 6px;
  }
  .row-gap {
    position: absolute;
    display: flex;
    column-gap: 10px;
    height: 10px;
  }
  .row-gap-segment {
    background: red;
    width: 44px;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
