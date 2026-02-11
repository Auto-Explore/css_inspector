# css/css-gaps/multicol/multicol-gap-decorations-017-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-017-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    .container {
        border: 2px solid rgb(96 139 168);
        width: 210px;
        height: 130px;
        column-gap: 20px;
        display: flex;
    }

    .items {
        background: rgb(96 139 168 / 0.2);
        height: 130px;
        margin: 0px;
        width: 56.666px;
    }

    .row-gap {
        position: absolute;
        display: flex;
        height: 10px;
        width: 210px;
        left: 2px;
        top: 62px;
        gap: 20px;
    }

    .row-item {
      height: 10px;
      background: gold;
      width: 60px;
    }

    .column-gap {
        position: absolute;
        height: 60px;
        width: 20px;
        background: blue;
        top: 2px;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
