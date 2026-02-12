# css/selectors/invalidation/state-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/state-in-has.html"
}
```

## style[0]

```css

  #subject {
    background-color: #f00;
  }
  #subject:has(:state(--green)) {
    background-color: #0f0;
  }
  #subject:has(:state(--blue)) {
    background-color: #00f;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
