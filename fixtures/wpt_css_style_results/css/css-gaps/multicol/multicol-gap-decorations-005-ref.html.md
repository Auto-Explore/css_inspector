# css/css-gaps/multicol/multicol-gap-decorations-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-005-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    .outer {
        display: flex;
        border: 1px solid #ccc;
        height: 50px;
        width: 210px;
        column-gap: 10px;
    }

    .outer-items {
        background: rgb(96 139 168 / 0.2);
        height 50px;
        width: 100px;
    }

    .inner {
        display: flex;
        height: 50px;
        column-gap: 10px;
        width: 100px;
    }

    .inner-items {
        height: 50px;
        width: 45px;
        background: black;
    }

    .column-gap {
        position: absolute;
        height: 50px;
        top: 1px;
        left: 101px;
        width: 10px;
    }

    .row-gap {
        position: absolute;
        background: purple;
        height: 10px;
        top: 21px;
        left: 111px;
        width: 100px;
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
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
