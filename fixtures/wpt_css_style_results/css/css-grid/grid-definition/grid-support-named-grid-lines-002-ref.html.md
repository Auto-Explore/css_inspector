# css/css-grid/grid-definition/grid-support-named-grid-lines-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-support-named-grid-lines-002-ref.html"
}
```

## style[0]

```css


.holder {
  width: 300px;
  height: 20px;
  border-bottom: 2px solid #cfbfcf;
}

.holder > :nth-child(2) {
  clear: left; /* Forces the div to a new line to simulate a new grid row. */
  padding-top: 2px; /* Simulates the grid row gap. */
}

.grid-container > * { float: left; height: 8px; }

.grid-container > :nth-child(3n)   {  background: sienna; }
.grid-container > :nth-child(3n+1) {  background: gold; }
.grid-container > :nth-child(3n+2) {  background: orange; }

.alt-color > :nth-child(2n) { background: sienna; }
.alt-color > :nth-child(2n+1) { background: orange; }

.invis { width: 0px; visibility: none; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
