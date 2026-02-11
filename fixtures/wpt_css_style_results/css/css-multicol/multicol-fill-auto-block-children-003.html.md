# css/css-multicol/multicol-fill-auto-block-children-003.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-fill-auto-block-children-003.html"
}
```

## style[0]

```css

  article {
    column-fill: auto;
    column-count: 2;
    width: 200px;
    /* Test max-height imposes constraint on column boxes' height. */
    max-height: 200px;
  }
  div {
    height: 400px;
    background-color: lightgreen;
  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
