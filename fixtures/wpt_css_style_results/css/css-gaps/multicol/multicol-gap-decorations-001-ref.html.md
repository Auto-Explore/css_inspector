# css/css-gaps/multicol/multicol-gap-decorations-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-001-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    .container {
        border: 2px solid rgb(96 139 168);
        width: 200px;
        height: 130px;
        column-gap: 10px;
        display: flex;
    }

    .items {
        background: rgb(96 139 168 / 0.2);
        height: 130px;
        margin: 0px;
        width: 60px;
    }

    .row-gap {
        position: absolute;
        height: 10px;
        width: 200px;
        background: gold;
        left: 2px;
        top: 62px;
    }

    .column-gap {
        position: absolute;
        height: 130px;
        width: 10px;
        background: blue;
        top: 2px;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
