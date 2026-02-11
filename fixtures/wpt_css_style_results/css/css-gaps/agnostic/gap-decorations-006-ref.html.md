# css/css-gaps/agnostic/gap-decorations-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/agnostic/gap-decorations-006-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

  .container {
    display: flex;
    width: 110px;
    height: 110px;
    column-gap: 10px;
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
    background: green;
    width: 110px;
    height: 10px;
  }

  .column-gap {
    position: absolute;
    top: 0px;
    left: 50px;
    background: pink;
    height: 110px;
    width: 10px;
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
