# css/css-gaps/flex/flex-gap-decorations-031.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-031.html"
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
        display: flex;
        column-gap: 10px;
        column-rule: 10px solid red;
        column-rule-break: intersection;
        column-rule-inset: 0;
        row-gap: 30px;
        row-rule: 30px solid blue;
        row-rule-break: intersection;
        row-rule-inset: 0;
        height: 130px;
        width: 230px;
        flex-wrap: wrap;
        align-content: center;
    }

    .items {
        width: 70px;
        height: 50px;
        writing-mode: vertical-rl;
    }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-inset”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-inset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
