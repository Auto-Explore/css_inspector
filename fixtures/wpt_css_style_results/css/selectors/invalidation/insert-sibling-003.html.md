# css/selectors/invalidation/insert-sibling-003.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/insert-sibling-003.html"
}
```

## style[0]

```css

    .c * { background-color: blue; }

    .a + * + .c * { background-color: green; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
