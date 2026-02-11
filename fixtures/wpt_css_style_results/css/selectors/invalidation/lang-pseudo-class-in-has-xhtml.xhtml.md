# css/selectors/invalidation/lang-pseudo-class-in-has-xhtml.xhtml

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/lang-pseudo-class-in-has-xhtml.xhtml"
}
```

## style[0]

```css

        body > *:not(p) { background: red; }
        section, div { width: 100px; height: 20px; }
        .lang:has(:lang(zh)) { background: green; }
        .lang:has(:lang(ja)) { background: red; }
        div:has(.lang:lang(fr)) { background: red; }
        div:has(.lang:lang(en)) { background: green; }
        div:has(#es:lang(es)) { background: red; }
        div:has(#es:lang(en)) { background: green; }
        .lang:has(#kr:lang(kr)) { background: red; }
        .lang:has(#kr:lang(kr)) { background: green; }
        div:has(.lang > :lang(pt)) { background: red; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
