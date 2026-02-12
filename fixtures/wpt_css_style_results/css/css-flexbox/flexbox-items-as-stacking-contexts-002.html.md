# css/css-flexbox/flexbox-items-as-stacking-contexts-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-items-as-stacking-contexts-002.html"
}
```

## style[0]

```css

    body { font: 10px sans-serif }
    .flexContainer {
      background: orange;
      display: flex;
      justify-content: space-between;
      width: 70px;
      padding: 2px;
      margin-bottom: 2px;
    }
    .item1 {
      background: lightblue;
      width: 30px;
      min-width: 0; /* disable default min-width:auto behavior */
    }
    .item2 {
      background: yellow;
      width: 30px;
      min-width: 0; /* disable default min-width:auto behavior */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
