# css/css-gaps/flex/flex-gap-decorations-022-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-022-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

  .container {
    display: flex;
    width: 120px;
    height: 110px;
    column-gap: 20px;
    row-gap: 10px;
    flex-wrap: wrap;
  }

  .item {
    background: skyblue;
    height: 50px;
    width: 50px;
    margin: 0;
  }

  .row-gap {
    position: absolute;
    top: 50px;
    background: gold;
    width: 50px;
    height: 10px;
  }

  .column-gap {
    position: absolute;
    left: 50px;
    background: blue;
    height: 50px;
    width: 20px;
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
