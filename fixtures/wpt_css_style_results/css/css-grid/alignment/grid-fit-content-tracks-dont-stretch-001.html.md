# css/css-grid/alignment/grid-fit-content-tracks-dont-stretch-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-fit-content-tracks-dont-stretch-001.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  width: 400px;
  height: 200px;
  font: 25px/1 Ahem;
}

.constrainedGrid {
  width: 20px;
  height: 10px;
}

.fitContent200x100 {
  grid-template-columns: fit-content(200px);
  grid-template-rows: fit-content(100px);
}

.placeContentStretch {
  place-content: stretch;
}

.placeContentStart {
  place-content: start;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
