# css/selectors/invalidation/media-pseudo-classes-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/media-pseudo-classes-in-has.html"
}
```

## style[0]

```css

  #subject {
    background-color: black;
    accent-color: black;
    color: black;
    border: 2px solid black;
  }
  #subject:has(:muted) {
    background-color: red;
  }
  #subject:has(:playing) {
    border-color: green;
  }
  #subject:has(:paused) {
    color: orange;
  }
  #subject:has(:seeking) {
    accent-color: blue;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
