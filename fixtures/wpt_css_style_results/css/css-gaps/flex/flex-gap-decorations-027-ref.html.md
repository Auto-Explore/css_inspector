# css/css-gaps/flex/flex-gap-decorations-027-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-027-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0 0 0 200px;
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
        width: 200px;
        flex-wrap: nowrap;
    }
    .items {
        width: 50px;
        height: 50px;
        flex-shrink: 0;
    }
    .column-gap {
        display: flex;
        margin-left: 50px;
        padding: 0px;
        height: 10px;
        width: 300px;
        position: absolute;
        top: 2px;
        height: 50px;
        left: 52px;
        column-gap: 50px;
    }
    .columns {
        background-color: red;
        width: 10px;
        height: 50px;
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
