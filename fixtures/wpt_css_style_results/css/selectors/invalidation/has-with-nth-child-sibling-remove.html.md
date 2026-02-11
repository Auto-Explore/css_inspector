# css/selectors/invalidation/has-with-nth-child-sibling-remove.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-with-nth-child-sibling-remove.html"
}
```

## style[0]

```css

.square {
  width: 100px;
  height: 100px;
  background: red;
}

.item:not(:has(~ .item > :nth-child(2))) {
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
