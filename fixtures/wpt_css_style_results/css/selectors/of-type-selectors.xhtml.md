# css/selectors/of-type-selectors.xhtml

```json
{
  "format_version": 3,
  "file": "css/selectors/of-type-selectors.xhtml"
}
```

## style[0]

```css

div > *|* {
  display: block;
  color: black;
  background: yellow;
  border: thin solid;
  margin: 1em;
}
.first-of-type > *|*:first-of-type {
  background: lime;
}
.nth-of-type > *|*:nth-of-type(1) {
  background: lime;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
