# css/css-grid/grid-lanes/tentative/track-sizing/grid-lanes-track-sizing-span-row-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/grid-lanes-track-sizing-span-row-ref.html"
}
```

## style[0]

```css

grid {
    display: grid;
    grid-template-columns: 50px 1fr;
    grid-template-rows: auto auto auto;
    width: 300px;
    height: 100px;
}

box1 {
    height: 50px;
    width: 50px;
    background-color: blue;
}

box2 {
    height: 50px;
    background-color: red;
}

box3 {
    width: 100px;
    height: 50px;
    background-color: purple;
    z-index: 1;
}

box4 {
    height: 50px;
    width: 300px;
    grid-column-start: 1;
    grid-column-end: 3;
    background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
