# css/css-gaps/multicol/multicol-gap-decorations-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-002-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    .outer {
        position: relative;
        border: 2px solid rgb(96 139 168);
        width: 200px;
        height: 200px;
    }

    .container {
        position: absolute;
        column-gap: 10px;
        display: flex;
    }

    .item {
        background: rgb(96 139 168 / 0.2);
        height: 100%;
        width: 60px;
    }

    .row-gap {
        position: absolute;
        height: 10px;
        width: 200px;
        background: gold;
        left: 0;
        top: 120px;
    }

    .column-gap {
        position: absolute;
        width: 10px;
        background: blue;
    }

    .spanner {
        position: absolute;
        background: cyan;
        width: 200px;
        height: 18px;
        top: 40px;
        left: 0;
        opacity: 0.5;
    }
```

```json
{
  "errors": 2,
  "messages": [
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
