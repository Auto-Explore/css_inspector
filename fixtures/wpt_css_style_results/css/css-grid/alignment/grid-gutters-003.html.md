# css/css-grid/alignment/grid-gutters-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-gutters-003.html"
}
```

## style[0]

```css

    #grid {
        display: grid;
        width:200px;
        gap: 40px 20px;
        grid-template-columns: 90px 90px;
        grid-template-rows: 90px 90px;
        background-color: green;
    }

    #grid > div {
        background-color: silver;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
