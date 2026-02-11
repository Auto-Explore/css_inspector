# css/css-grid/grid-items/overflow-hidden-min-content-contribution-in-abs-pos-grid-in-flexbox.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/overflow-hidden-min-content-contribution-in-abs-pos-grid-in-flexbox.html"
}
```

## style[0]

```css

    .flexbox {
        display: flex;
        width: 100px;
        height: 100px;
    }
    .flex-item {
        flex-grow: 1;
        position: relative;
    }
    .grid {
        display: grid;
        grid-template-rows: auto 1fr;
        font: 100px/1 Ahem;
        color: green;
        position: absolute;
        top: 0;
        bottom: 0;
    }
    .grid-item {
        overflow: hidden;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
