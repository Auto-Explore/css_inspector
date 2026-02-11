# css/css-gaps/grid/grid-gap-decorations-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-003-ref.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }

  .container {
    height: 110px;
    width: 110px;

    display: grid;
    grid-template-columns: repeat(2, 1fr);

    column-gap: 10px;
    row-gap: 10px;
  }


  .item {
    background: skyblue;
    height: 50px;
    width: 100%;
    margin: 0;
  }

  .col-rule {
    margin: 0px;
    padding: 0px;
    width: 0px;


    height: 110px;
    border-right: 10px dotted;
    border-color: pink;
    position: absolute;
    left: 50px;
    top: 0px;

  }

  .row-rule {
    margin: 0px;
    padding: 0px;
    height: 0px;

    width: 110px;
    border-bottom: 10px dotted;
    border-color: green;
    position: absolute;
    top: 50px;
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
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
