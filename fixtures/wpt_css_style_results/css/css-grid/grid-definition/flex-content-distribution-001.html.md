# css/css-grid/grid-definition/flex-content-distribution-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/flex-content-distribution-001.html"
}
```

## style[0]

```css

.freeSpaceForColumnsGrid {
    grid-template: 100% / minmax(20px, 0.7fr);
    width: 50px;
    height: 100px;
}

.freeSpaceForRowsGrid {
    grid-template: minmax(20px, 0.7fr) / 100%;
    width: 50px;
    height: 100px;
}

.container { position: relative; }

.item {
    width: 100%;
    height: 50px;
    background-color: red;
}

.item2 {
    width: 50px;
    height: 100%;
    background-color: red;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
