# css/selectors/invalidation/lang-pseudo-class-in-has-document-element.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/lang-pseudo-class-in-has-document-element.html"
}
```

## style[0]

```css

        div { width: 100px; height: 100px; }
        div:has(*:lang(fr)) { background: red; }
        div:has(*:lang(en)) { background: green; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
