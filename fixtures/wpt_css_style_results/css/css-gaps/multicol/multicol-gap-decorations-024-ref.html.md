# css/css-gaps/multicol/multicol-gap-decorations-024-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-024-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    .container {
        width: 200px;
        height: 150px;
        column-count: 3;
        column-width: 60px;
        column-height: 70px;
        column-gap: 10px;
        row-gap: 10px;
        column-wrap: wrap;
    }

    .items {
        background: rgb(96 139 168 / 0.2);
        height: 70px;
        width: 60px;
        margin: 0px;
        writing-mode: vertical-rl;
    }

    .col-rule {
        position: absolute;
        top: 0px;
        height: 150px;
        width: 10px;
        background: blue;
    }

    .row-rule {
        position: absolute;
        height: 10px;
        width: 200px;
        background: gold;
        left: 0px;
        top: 70px;
    }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “column-height”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-wrap”.",
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
