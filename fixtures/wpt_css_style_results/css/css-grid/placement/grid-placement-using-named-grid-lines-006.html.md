# css/css-grid/placement/grid-placement-using-named-grid-lines-006.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-placement-using-named-grid-lines-006.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  width: 300px;
  height: 300px;
  margin-left: -100px;
  margin-top: -100px;
  grid-template-areas: ". ." ". foo";
  grid-template-columns: repeat(auto-fill, 100px) [foo-start];
  grid-template-rows: repeat(auto-fill, 100px) [foo-start];
}
.grid > div {
  grid-area: 1 foo-start / 1 foo-start;
  background: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid-template-areas”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
