# css/css-overflow/line-clamp/discard/discard-multicol-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/discard/discard-multicol-003.html"
}
```

## style[0]

```css

div {
    columns: 3;
    border: solid 1px;
}
div p {
    break-after: column;
}
.spanner {
    column-span: all;
    text-align: center;
}

.test {
    continue: discard;
    block-overflow: no-ellipsis;
}

/* incidental to the example, but needed for a controlled rendering :*/
div p { margin: 0; }
div {
    margin: 1em;
    width: 47ch;
    font-family: monospace;
    gap: 1ch;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
