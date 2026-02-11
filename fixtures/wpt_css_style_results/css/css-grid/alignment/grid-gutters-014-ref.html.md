# css/css-grid/alignment/grid-gutters-014-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-gutters-014-ref.html"
}
```

## style[0]

```css

    #grid {
        border: solid 5px teal;
        width: 200px;
        height: 100px;
        padding: 10px 0 0 10px;
    }

    #grid > div {
        float: left;
        border: solid 5px aqua;
        width: 70px;
        height: 30px;
        margin-left: 5px;
    }

    /* highlight manual pass condition */
    #grid > div:nth-child(even) {
        border-right: none;
        margin-left: 40px;
    }
    #grid > div:nth-child(n + 3) {
        border-bottom: none;
        margin-top: 25px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
