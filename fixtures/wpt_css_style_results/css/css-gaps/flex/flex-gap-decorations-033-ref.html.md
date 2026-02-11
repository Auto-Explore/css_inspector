# css/css-gaps/flex/flex-gap-decorations-033-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-033-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }
    #flexbox {
        display: flex;
        width: 150px;
        flex-wrap: wrap;
    }
    .items {
        background-color: rgb(96 139 168 / 0.2);
        width: 50px;
        height: 50px;
    }
    #four {
        width: 100px;
    }
    .row-gap {
        margin: 0px;
        padding: 0px;
        height: 10px;
        background: blue;
        width: 150px;
        position: absolute;
        z-index: -1;
    }
    .column-gap {
        display: flex;
        flex-direction: column;
        row-gap: 50px;
        height: 150px;
        top: 0px;
        width: 10px;
        left: 45px;
        position: absolute;
        z-index: -1;
    }
    .column {
        background: red;
        width: 10px;
        height: 50px;
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
