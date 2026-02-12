# css/css-grid/grid-lanes/tentative/abspos/row-grid-lanes-intrinsic-sizing-oof-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/abspos/row-grid-lanes-intrinsic-sizing-oof-ref.html"
}
```

## style[0]

```css

    .container {
        border: 2px solid black;
        margin: 10px;
        height: 300px;
        position: relative;
    }

    .grid {
        display: grid;
        border: 2px solid blue;
        padding: 5px;
        grid-template-columns: auto;
        grid-template-rows: repeat(3, auto);
    }

    .oof-item {
        position: absolute;
        background: red;
        color: white;
        padding: 5px;
        border: 1px solid darkred;
    }

    .oof-1 {
        top: 50px;
        left: 50px;
        width: 60px;
        height: 40px;
    }

    .oof-2 {
        top: 150px;
        left: 100px;
        width: 70px;
        height: 50px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
