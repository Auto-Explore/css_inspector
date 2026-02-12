# css/css-grid/grid-definition/grid-auto-repeat-multiple-values-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-auto-repeat-multiple-values-002-ref.html"
}
```

## style[0]

```css


.grid-container {
  height: 30px;
  width: 300px;
  border-bottom: 2px solid #cfbfcf;
}

.grid-container > * { float: left; height: 30px; }
.grid-container > :nth-child(2n) {  background: sienna; }
.grid-container > :nth-child(2n + 1) {  background: orange; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
