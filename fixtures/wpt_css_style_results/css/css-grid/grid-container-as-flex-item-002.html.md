# css/css-grid/grid-container-as-flex-item-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-container-as-flex-item-002.html"
}
```

## style[0]

```css

.inner-grid {
  display: grid;
  flex: 1 1 0%;
  width: 100%;
  min-height: 0; /* Disable automatic minimum size in the main axis */
  background: red;
}

.inner-grid-item {
  font: 50px/1 Ahem;
  /* Disable automatic minimum size in inline axis. This is essential to
     reproduce this bug. 'min-width:auto' doesn't reproduce. */
  min-width: 0;
  width: 100%;
  color: transparent;
  background: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
