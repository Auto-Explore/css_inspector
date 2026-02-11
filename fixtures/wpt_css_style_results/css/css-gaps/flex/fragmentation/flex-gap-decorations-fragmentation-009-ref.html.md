# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-009-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-009-ref.html"
}
```

## style[0]

```css

    .multi-column {
        columns: 2;
        height: 90px;
        column-width: 100px;
        width: 330px;
    }

    body {
        margin: 0px;
    }

    #flexbox {
        border: 2px solid rgb(96 139 168);
        display: flex;
        column-gap: 10px;
        row-gap: 10px;
        width: 90px;
        flex-wrap: wrap;
        height: 170px;
        flex-direction: column;
    }

    .items {
        background-color: rgb(96 139 168 / 0.2);
        width: 30px;
        height: 30px;
        break-before: avoid-column;
    }

    .row-rule {
        background-color: gold;
        height: 10px;
        width: 30px;
        position: absolute;
        width: 90px;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
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
