# css/css-gaps/grid/grid-gap-decorations-repaint-on-child-resize.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-repaint-on-child-resize.html"
}
```

## style[0]

```css

    .grid-container {
      display: grid;
      grid-template-columns: auto auto;
      column-gap: 20px;
      width: 160px;
      height: 50px;
      background: red;
      column-rule: 20px solid green;
    }

    .grid-item {
      background: green;
      height: 50px;
    }

    #item1 {
      /* Start at 40px, will change to 90px */
      width: 40px;
    }

    #item2 {
      width: 50px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
