# css/css-grid/abspos/positioned-grid-items-should-not-take-up-space-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-should-not-take-up-space-001.html"
}
```

## style[0]

```css


.grid {
  grid-template-columns: 50px 100px;
  grid-template-rows: 50px 100px;
  width: 150px;
  height: 150px;
  /* Ensures that the grid container is the containing block of the absolutely positioned grid children. */
  position: relative;
  /* Ensures that the grid container is the containing block of the fixed positioned grid children. */
  transform: translate(10px, 20px);
}

.absolute {
  position: absolute;
}

.fixed {
  position: fixed;
}

.offsetLeft100 {
  left: 100px;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
