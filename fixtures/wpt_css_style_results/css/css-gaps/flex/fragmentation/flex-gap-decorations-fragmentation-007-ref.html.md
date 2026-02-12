# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-007-ref.html"
}
```

## style[0]

```css

    .multi-column {
        columns: 2;
        column-width: 90px;
        width: 330px;
        height: 130px;
        column-fill: balance;
    }

    body {
        margin: 0px;
    }

    #flexbox {
        border: 2px solid rgb(96 139 168);
        display: flex;
        column-gap: 10px;
        row-gap: 20px;
        width: 90px;
        flex-wrap: wrap;
        height: 130px;
        align-items: center;
        justify-content: center;
    }

    .items {
        background-color: rgb(96 139 168 / 0.2);
        width: 30px;
        height: 30px;
    }

    .column-rule {
        background-color: blue;
        position: absolute;
        height: 65px;
        width: 10px;
    }

    .row-rule {
        background-color: gold;
        position: absolute;
        height: 20px;
        width: 90px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
