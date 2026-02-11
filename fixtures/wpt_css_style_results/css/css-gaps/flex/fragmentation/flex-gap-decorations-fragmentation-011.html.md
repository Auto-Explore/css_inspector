# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-011.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-011.html"
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
        height: 110px;
        column-rule: 10px solid blue;
        row-rule: 10px solid gold;
    }

    .items {
        background-color: rgb(96 139 168 / 0.2);
        width: 50px;
        height: 50px;
    }

    #one {
        height: 25px;
    }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
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
