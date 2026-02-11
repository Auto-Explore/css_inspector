# css/selectors/invalidation/has-with-nth-child.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-with-nth-child.html"
}
```

## style[0]

```css

  #test-container > div { background-color: green; }
  #target1:has(.item:nth-child(3)) { background-color: red; }
  #target2:has(.item:nth-last-child(3)) { background-color: red; }
  #target3:has(.item:nth-child(3) > .child) { background-color: red; }
  #target4:has(.item:nth-last-child(3) > .child) { background-color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
