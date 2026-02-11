# css/selectors/invalidation/lang-pseudo-class-in-has-multiple-document-elements.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/lang-pseudo-class-in-has-multiple-document-elements.html"
}
```

## style[0]

```css

    div { width: 100px; height: 100px; }
    div:has(*:lang(fr)) { background: red; }
    div:has(*:lang(en)) { background: green; }
    span.pass { color: green; }
    span.fail { color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
