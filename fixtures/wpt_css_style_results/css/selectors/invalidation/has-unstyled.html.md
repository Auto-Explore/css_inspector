# css/selectors/invalidation/has-unstyled.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-unstyled.html"
}
```

## style[0]

```css

div, main { color: grey }
.none { display: none; }
#subject:has(.test) { color: red; }
#subject:has(~ #sibling .test) { color: green; }
#subject:has(:is(.test_inner #subject_descendant)) { color: blue; }
#subject:has(~ #sibling :is(.test_inner #sibling_descendant)) { color: yellow; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
