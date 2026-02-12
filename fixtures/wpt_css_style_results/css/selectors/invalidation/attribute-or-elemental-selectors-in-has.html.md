# css/selectors/invalidation/attribute-or-elemental-selectors-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/attribute-or-elemental-selectors-in-has.html"
}
```

## style[0]

```css

div, main { color: grey }
.subject:has(> .child) { color: red }
.subject:has(.descendant) { color: green }
.subject:has([attrname=descendant]) { color: blue }
.subject:has(#div_descendant) { color: yellow }
.subject:has(descendant) { color: yellowgreen }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
