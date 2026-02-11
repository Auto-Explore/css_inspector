# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-001.html"
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
        row-rule-break: intersection;
        row-rule-edge-inset-end: 1px;
        row-rule-edge-inset-start: 2px;
        column-rule-style: solid;
        column-rule-color: red;
        column-rule-width: 10px;
        column-rule-break: intersection;
        column-rule-edge-inset-start: 1px;
        column-rule-edge-inset-end: -1px;
        column-rule-interior-inset-start: 2px;
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
  "errors": 12,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
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
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-start”.",
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
