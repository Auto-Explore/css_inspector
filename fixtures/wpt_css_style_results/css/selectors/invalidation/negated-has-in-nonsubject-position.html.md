# css/selectors/invalidation/negated-has-in-nonsubject-position.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/negated-has-in-nonsubject-position.html"
}
```

## style[0]

```css

.c {
  width: 100px;
  height: 100px;
  background: green;
}
.b:not(.ancestor:has(+ .sibling) .a) .c {
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
