# css/css-text/white-space/reference/text-wrap-balance-004-a-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/reference/text-wrap-balance-004-a-ref.html"
}
```

## style[0]

```css

div {
    border: solid;
    font-family: monospace;
    margin: 1ch;
    width: 8.5ch;
    /* the .5ch above should not be necessary,
       but in some browsers the ellipsis is a little larger than 1ch,
       so this gives it room to breathe.
       Needing this may or may not be a bug,
       but this is not what we're testing here.
     */
}
.test {
    border-color: blue;
}
.ref1 {
    border-color: orange;
}

.ref2 {
    border-color: magenta;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
