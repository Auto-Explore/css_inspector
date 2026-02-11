# css/selectors/invalidation/lang-pseudo-class-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/lang-pseudo-class-in-has.html"
}
```

## style[0]

```css

        div { width: 100px; height: 20px; }
        .lang div:lang(en) { background: green; }
        .lang div:lang(fr) { background: red; }
        .lang div:lang(ja) { background: red; }
        div:has(.lang :lang(zh)) { background: red; }
        div:has(.lang :lang(en)) { background: green; }
        .lang:lang(kr) { background: red; }
        .lang div:lang(kr) { background: green; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
