# css/css-grid/grid-lanes/tentative/alignment/column-grid-lanes-alignment-positioned-items-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/column-grid-lanes-alignment-positioned-items-003-ref.html"
}
```

## style[0]

```css

    html,body {
        color:black; background-color:white; font:8px/1 monospace; padding:0; margin:0;
    }

    .grid {
        position: relative;
        display: grid;
        grid-template-columns: 90px 90px 90px;
        grid-template-rows: 150px;
        width: 320px;
        gap: 10px;
        padding: 15px;
        border: 2px solid black;
        margin: 10px;
    }

    .grid > div {
        position: absolute;
        border: 1px solid #333;
        width: 70px;
        height: 30px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .flex-start-item {
        grid-column: 1 / 2;
        align-self: flex-start;
        justify-self: flex-start;
        background: lightcoral;
    }

    .flex-end-item {
        grid-column: 2 / 3;
        align-self: flex-end;
        justify-self: flex-end;
        background: lightblue;
    }

    .mixed-legacy {
        grid-column: 3 / 4;
        align-self: flex-start;
        justify-self: end;
        background: lightgreen;
    }

    .normal-item {
        grid-column: 1 / 2;
        align-self: normal;
        justify-self: normal;
        background: lightyellow;
    }

    .auto-item {
        grid-column: 2 / 3;
        align-self: auto;
        justify-self: auto;
        background: plum;
    }

    .legacy-center {
        grid-column: 3 / 4;
        align-self: flex-start;
        justify-self: center;
        background: orange;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
