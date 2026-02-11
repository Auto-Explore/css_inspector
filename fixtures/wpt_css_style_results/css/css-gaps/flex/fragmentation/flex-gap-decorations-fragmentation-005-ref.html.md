# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-005-ref.html"
}
```

## style[0]

```css

    .multi-column {
        columns: 3;
        height: 47px;
        column-width: 110px;
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
        width: 110px;
        flex-wrap: wrap;
        flex-direction: column;
        height: 110px;
    }

    .items {
        background-color: rgb(96 139 168 / 0.2);
        width: 50px;
        height: 50px;
    }

    .column-rule {
        background-color: blue;
        position: absolute;
        top: 0px;
        height: 47px;
        width: 10px;
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
