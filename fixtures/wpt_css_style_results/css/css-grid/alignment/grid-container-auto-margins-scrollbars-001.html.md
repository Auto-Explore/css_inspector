# css/css-grid/alignment/grid-container-auto-margins-scrollbars-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-container-auto-margins-scrollbars-001.html"
}
```

## style[0]

```css

body {
   display: grid;
   grid-template-columns: auto;
   grid-template-rows: 1fr auto;
   height: 100vh;
}
.item1 {
   margin: 0px auto;
}
.item2 {
   background-color: cyan;
   height: 50px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
