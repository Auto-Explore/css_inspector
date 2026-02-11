# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-004-ref.html"
}
```

## style[0]

```css

    .multi-column {
        columns: 3;
        height: 47px;
        column-width: 170px;
        width: 510px;
    }

    body {
        margin: 0px;
    }

    #flexbox {
        border: 2px solid rgb(96 139 168);
        display: flex;
        column-gap: 10px;
        row-gap: 10px;
        width: 170px;
        flex-wrap: wrap;
    }

    .items {
        background-color: rgb(96 139 168 / 0.2);
        flex-shrink: 1;
        width: 50px;
        height: 50px;
    }

    #four {
        width: 100px;
    }

    .column-rule {
        position: absolute;
        top: 1px;
        background: red;
        width: 10px;
    }

    .row-rule {
        position: absolute;
        background: blue;
        width: 170px;
        height: 10px;
        top: 5px;
        left: 265px;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
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
