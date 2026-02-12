# css/selectors/invalidation/has-with-is-child-combinator.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-with-is-child-combinator.html"
}
```

## style[0]

```css

div, main { color: grey }
#subject:has(> :is(.parent > .child)) { color: green }
#subject:has(> :is(.parent .descendant)) { color: blue }
#subject:has(:is(.deep-parent > .deep-child)) { color: red }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
