# css/css-grid/grid-lanes/tentative/item-placement/row-auto-placement-max-content-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/row-auto-placement-max-content-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:15px/1 monospace; padding:0; margin:0;
}

.grid {
    display: grid;
    grid-template-rows: 15px auto auto;
    align-items: start;
    background: gray;
    padding: 10px;
    width: max-content;
}

.flex {
    display: flex;
    flex-direction: row;
    overflow: visible;
    flex-wrap: nowrap;
    width: max-content;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
