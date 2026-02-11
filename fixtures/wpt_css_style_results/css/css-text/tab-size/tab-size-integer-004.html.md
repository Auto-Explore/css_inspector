# css/css-text/tab-size/tab-size-integer-004.html

```json
{
  "format_version": 3,
  "file": "css/css-text/tab-size/tab-size-integer-004.html"
}
```

## style[0]

```css

div {
    white-space: pre;
    tab-size: 2; /* Overridden, as tab-size applies to inline boxes */
}
span {
    tab-size: 4;

    /* Various factors that are taken into account in the calculation of the tab size,
       but do not matter here because it must be calculated based on the block container
       rather than the inline.
    */
    font-size: 2em;
    font-family: monospace;
    letter-spacing: 5px;
    word-spacing: 5px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
