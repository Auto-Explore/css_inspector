# css/selectors/invalidation/has-pseudoclass-only.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-pseudoclass-only.html"
}
```

## style[0]

```css

#subject {
  color: grey;
}

#subject.empty:has(:empty) {
  color: red;
}

#subject.first:has(:first-child) {
  color: orange;
}

#subject.last:has(:last-child) {
  color: yellow;
}

/* :empty :empty never matches, so use something else. */
#subject.empty-subtree:has(:first-child :empty) {
  color: green;
}

#subject.first-subtree:has(:first-child :first-child) {
  color: blue;
}

#subject.last-subtree:has(:last-child :last-child) {
  color: purple;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
