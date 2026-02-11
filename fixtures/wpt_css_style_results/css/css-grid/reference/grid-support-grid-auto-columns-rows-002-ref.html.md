# css/css-grid/reference/grid-support-grid-auto-columns-rows-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/reference/grid-support-grid-auto-columns-rows-002-ref.html"
}
```

## style[0]

```css

    #grid {
        position: relative;
    }

    .absolute {
        position: absolute;
        top: 0;
        left: 0;
    }

    #first-column-first-row {
        width: 25px;
        height: 40px;
        background-color: purple;
    }

    #second-column-first-row {
        width: 50px;
        height: 40px;
        left: 25px;
        background-color: orange;
    }

    #first-column-second-row {
        width: 25px;
        height: 30px;
        top: 40px;
        background-color: green;
    }

    #second-column-second-row {
        width: 50px;
        height: 30px;
        top: 40px;
        left: 25px;
        background-color: pink;
    }

    #first-and-second-column-third-row {
        width: 75px;
        height: 40px;
        top: 70px;
        background-color: silver;
    }

    #third-column-all-rows {
        left: 75px;
        height: 110px;
        width: 25px;
        background-color: blue;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
