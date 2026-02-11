# css/selectors/invalidation/any-link-attribute-removal.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/any-link-attribute-removal.html"
}
```

## style[0]

```css

  span { color: green; }
  :any-link + span { color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
