# css/css-grid/abspos/positioned-grid-descendants-017-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-descendants-017-ref.html"
}
```

## style[0]

```css

  #grid {
    display: inline-grid;
    grid-template-columns: 50px 50px;
    grid-template-rows: 50px 50px;
    background-color: blue;
  }
  #child-1 {
    grid-area: 1 / 1 / 1 / 1;
    width: 50px;
    height: 50px;
    background-color: hotpink;
  }
  #child-2 {
    grid-area: 2 / 2 / 2 / 2;
    width: 50px;
    height: 50px;
    background-color: green;
  }
  #grandchild {
    width: 25px;
    height: 25px;
    background-color: red;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid-area”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
