# css/css-flexbox/flexbox-safe-overflow-position-006.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-safe-overflow-position-006.html"
}
```

## style[0]

```css

  #reference-overlapped-red {
    position: absolute;
    background-color: red;
    width: 100px;
    height: 100px;
    z-index: -1;
  }

  .flex {
    display: -webkit-box;
    -webkit-box-align: end;
    width: 90px;
    height: 90px;
    /* Make the green square cover the red square. */
    translate: 0 10px;
  }

  .item {
    min-width: 100px;
    min-height: 100px;
    background: green;
    align-self: safe center;
  }

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-box-align”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
