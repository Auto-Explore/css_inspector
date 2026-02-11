# css/css-gaps/multicol/multicol-gap-decorations-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-006-ref.html"
}
```

## style[0]

```css

    .outer-container {
        position: relative;
        columns: 3;
        column-fill: auto;
        height: 100px;
        width: 620px;
        gap: 10px;
        background: yellow;
    }

    .nested-container {
        columns: 2;
        box-decoration-break: clone;
        padding: 10px;
        gap: 10px;
        background: cyan;
    }

    .inner-column {
        height: 250px;
        background: hotpink;
    }

    #col-gap1 {
        position: absolute;
        height: 80px;
        width: 1px;
        background: green;
        top: 10px;
        left: 100px;
    }

    #col-gap2 {
        position: absolute;
        width: 1px;
        background: green;
        top: 10px;
        left: 310px;
        height: 45px;
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
