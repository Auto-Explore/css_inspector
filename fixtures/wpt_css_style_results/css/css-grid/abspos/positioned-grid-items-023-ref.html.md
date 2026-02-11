# css/css-grid/abspos/positioned-grid-items-023-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-023-ref.html"
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
  #child {
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
