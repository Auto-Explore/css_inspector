# css/css-writing-modes/reference/table-cell-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/reference/table-cell-002-ref.html"
}
```

## style[0]

```css

div {
  font: 50px / 50px ahem;
  color: green;

  max-height: 100px;
  writing-mode: vertical-rl;
  /* Doing this with a horizontal div and max-width ought to result in the same 100x100 green square,
     but font rasterisation may (and in practice does) result in different aliasing artifacts.
     Therefore I am using the same writing mode as the test,
     to make sure we get identically sharp/fuzzy edges on the same sides.
  */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
