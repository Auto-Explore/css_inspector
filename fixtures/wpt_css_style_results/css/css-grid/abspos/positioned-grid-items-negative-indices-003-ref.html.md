# css/css-grid/abspos/positioned-grid-items-negative-indices-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-negative-indices-003-ref.html"
}
```

## style[0]

```css

  #grid {
    display: grid;
    margin: 4px;
    padding: 10px;
    width: 500px;
    height: 130px;
    position: relative;
  }

  #firstItem {
    grid-column: 1 / span 2;
    background-color: hotpink;
  }

  #secondItem {
    grid-column: 3 / span 6;
    background-color: lightblue;
  }

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
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
