# css/selectors/has-style-sharing-006.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-style-sharing-006.html"
}
```

## style[0]

```css

div {
  background: blue;
  margin: 1em;
  padding: 1em;
}

span {
  display: inline-block;
  width: 1em;
  height: 1em;
}

:has(> .a) .b .c {
  background: green;
}

.c {
  background: purple;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
