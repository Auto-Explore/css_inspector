# css/css-grid/abspos/positioned-grid-items-023.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-023.html"
}
```

## style[0]

```css

  #grid {
    display: inline-grid;
    grid-template-columns: 50px 50px;
    grid-template-rows: 50px 50px;
    background-color: blue;
    position: relative;
  }
  #child {
    grid-area: 2 / 2 / 2 / 2;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: green;
    contain: strict;
    position: absolute;
  }
  #grandchild {
    width: 50%;
    height: 50%;
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
