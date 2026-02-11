# css/css-gaps/flex/flex-gap-decorations-018.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-018.html"
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
        row-gap: 20px;
        width: 50px;
        height: 190px;
        flex-wrap: wrap;
        flex-direction: column;
        row-rule-color: red;
        row-rule-width: 20px;
        row-rule-style: outset;
    }

    .items {
        background-color: rgb(96 139 168 / 0.2);
        flex-shrink: 1;
        width: 50px;
        height: 50px;
    }

```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
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
      "message": "Unknown property “row-rule-style”.",
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
