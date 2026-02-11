# css/css-gaps/flex/flex-gap-decorations-007.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-007.html"
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
        width: 170px;
        flex-wrap: wrap;
        align-items: flex-end;
    }

    .items {
        width: 50px;
    }

    #three {
        height: 40px;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
