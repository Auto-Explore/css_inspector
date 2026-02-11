# css/selectors/invalidation/has-pseudo-element.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-pseudo-element.html"
}
```

## style[0]

```css

.anchor:has(.foo)::before {
  content: "";
  display: block;
  width: 100px;
  height: 100px;
  background: green;
  position: absolute;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
