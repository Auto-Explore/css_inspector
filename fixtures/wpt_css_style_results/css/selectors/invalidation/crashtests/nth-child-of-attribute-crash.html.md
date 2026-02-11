# css/selectors/invalidation/crashtests/nth-child-of-attribute-crash.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/crashtests/nth-child-of-attribute-crash.html"
}
```

## style[0]

```css

#elements [blue] {
    color: blue;
}
#elements :nth-child(1 of :not([blue])) {
    color: revert;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
