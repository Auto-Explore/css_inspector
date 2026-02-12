# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-012-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-012-ref.html"
}
```

## style[0]

```css

    .multi-column {
        columns: 3;
        height: 47px;
        column-width: 110px;
        width: 330px;
        column-fill: auto;
    }

    body {
        margin: 0px;
    }

    #flexbox {
        xborder: 2px solid rgb(96 139 168);
        display: flex;
        column-gap: 10px;
        row-gap: 10px;
        width: 110px;
        flex-wrap: wrap;
        height: 110px;
    }

    .items {
        background-color: rgb(96 139 168 / 0.2);
        width: 50px;
        height: 55px;
    }

    #monolithic {
        contain: size;
    }

    .row-rule {
        position: absolute;
        background-color: gold;
        height: 10px;
    }

    .column-rule {
        position: absolute;
        background-color: blue;
        width: 10px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
