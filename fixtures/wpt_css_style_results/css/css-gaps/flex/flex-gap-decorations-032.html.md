# css/css-gaps/flex/flex-gap-decorations-032.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-032.html"
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
        row-gap: 30px;
        row-rule: 30px solid blue;
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
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
