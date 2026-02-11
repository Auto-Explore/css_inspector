# css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-014-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/fragmentation/flex-gap-decorations-fragmentation-014-ref.html"
}
```

## style[0]

```css

    .multi-column {
        columns: 3;
        height: 47px;
        column-width: 110px;
        width: 330px;
        column-fill: auto;
    }

    body {
        margin: 0px;
    }

    #flexbox {
        display: flex;
        column-gap: 10px;
        row-gap: 10px;
        width: 110px;
        flex-wrap: wrap;
        height: 110px;
        flex-direction: column;
    }

    .items {
        background-color: rgb(96 139 168 / 0.2);
        width: 50px;
        height: 50px;
    }

    #monolithic {
        contain: size;
    }

    .column-rule {
        position: absolute;
        background-color: blue;
        top: 0px;
        width: 10px;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
