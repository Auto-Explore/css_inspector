# css/css-gaps/flex/flex-gap-decorations-006.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-006.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    #flexbox>* {
        background-color: rgb(96 139 168 / 0.2);
    }

    #flexbox {
        border: 2px solid rgb(96 139 168);
        border-width: 2px;
        display: flex;
        column-gap: 10px;
        column-rule-style: solid;
        column-rule-width: 10px;
        column-rule-color: red;
        row-gap: 30px;
        row-rule-style: solid;
        row-rule-width: 30px;
        row-rule-color: blue;
        height: 300px;
        width: 300px;
        flex-wrap: wrap;
        align-content: center;
        writing-mode: vertical-lr;
    }

    .items {
        width: 70px;
        height: 70px;
    }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
