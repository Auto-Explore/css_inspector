# css/css-grid/abspos/positioned-grid-items-should-not-create-implicit-tracks-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-should-not-create-implicit-tracks-001.html"
}
```

## style[0]

```css


.grid {
  grid-auto-columns: 100px;
  grid-auto-rows: 50px;
  width: 400px;
  height: 300px;
  /* Ensures that the grid container is the containing block of the absolutely positioned grid children. */
  position: relative;
}

.absolute {
  position: absolute;
}

.seventhRowFourthColumn {
  background-color: coral;
  grid-column: 4;
  grid-row: 7;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
