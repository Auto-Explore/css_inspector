# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-004.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-004.html"
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
        row-rule-style: solid;
        row-rule-color: blue;
        row-rule-width: 10px;
        column-rule-style: solid;
        column-rule-color: red;
        column-rule-width: 10px;
        column-rule-break: intersection;
        column-rule-inset: 1px;
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
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-inset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
