# css/css-align/self-alignment/block-justify-self.html

```json
{
  "format_version": 3,
  "file": "css/css-align/self-alignment/block-justify-self.html"
}
```

## style[0]

```css

.grid-container div {
  width: 100px;
  background-color: green;
}

.block-container div, .block-container-with-table-items div {
  width: 100px;
  background-color: red;
}

.stack {
  display: grid;
}

.stack > * {
  grid-row: 1;
  grid-column: 1;
}

body {
  width: 700px;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
