# css/css-gaps/grid/grid-gap-decorations-052-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-052-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }
    .grid {
        display: grid;
        grid-template: repeat(auto-fill, 100px) / repeat(3, 100px);
        grid-gap: 20px;
        left: 0px;
        top: 0px;
    }
    .item {
        width: 100px;
        height: 100px;
        background: lightgray;
    }
    .column-decoration {
        position: absolute;
        top: 0px;
        height: 340px;
        width: 3px;
        background: red;
        height: 105px;
    }
    .row-decoration {
        position: absolute;
        height: 3px;
        background-color: gray;
        width: 100px;
        left: 1px;
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
