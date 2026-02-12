# css/selectors/invalidation/media-loading-pseudo-classes-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/media-loading-pseudo-classes-in-has.html"
}
```

## style[0]

```css

#subject:has(video:buffering) {
    background-color: blue;
}
#subject:has(video:stalled) {
    border-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
