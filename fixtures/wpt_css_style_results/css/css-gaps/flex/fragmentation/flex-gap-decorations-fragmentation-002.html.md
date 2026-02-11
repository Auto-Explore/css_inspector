# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-002.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }
    .flex-container {
        height: 110px;
        width: 110px;
        display: flex;
        flex-wrap: wrap;

        gap: 10px;
        column-rule: pink solid 10px;
        row-rule: green solid 10px;
    }
    .flex-item {
        background: skyblue;
        width: 50px;
        height: 50px;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
