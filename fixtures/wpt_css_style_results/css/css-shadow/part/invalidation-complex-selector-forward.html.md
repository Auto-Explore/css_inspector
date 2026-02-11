# css/css-shadow/part/invalidation-complex-selector-forward.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/invalidation-complex-selector-forward.html"
}
```

## style[0]

```css
#elem #c-e-outer::part(part-forwarded) { color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css
span { color: green; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
