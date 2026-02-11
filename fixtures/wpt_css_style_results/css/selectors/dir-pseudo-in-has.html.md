# css/selectors/dir-pseudo-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/dir-pseudo-in-has.html"
}
```

## style[0]

```css

        div, section { width: 100px; height: 20px; }
        section { background: red; }
        .ltr:has(*:dir(ltr)) { background: green; }
        .ltr:has(*:dir(rtl)) { background: red; }
        .rtl:has(*:dir(rtl)) { background: green; }
        .rtl:has(*:dir(ltr)) { background: red; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
