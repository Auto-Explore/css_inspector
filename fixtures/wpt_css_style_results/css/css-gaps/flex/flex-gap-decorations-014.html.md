# css/css-gaps/flex/flex-gap-decorations-014.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-014.html"
}
```

## style[0]

```css

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
        row-rule-width: 2px;
        column-rule-style: solid;
        column-rule-color: red;
        column-rule-width: 2px;
        column-rule-break: intersection;
        column-rule-inset: -100%;
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
