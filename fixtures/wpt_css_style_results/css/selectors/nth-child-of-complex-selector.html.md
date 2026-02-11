# css/selectors/nth-child-of-complex-selector.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-of-complex-selector.html"
}
```

## style[0]

```css

/* At least 4 pair of <p> above, can be overlapping. The other selectors of the list are useless. */
p:nth-child(4n of html:root>body>p+p:not(empty), :not(*), p:not(p), span, .notthere) {
    background-color: lime;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
