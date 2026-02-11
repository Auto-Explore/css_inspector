# css/css-content/attr-case-sensitivity-001.html

```json
{
  "format_version": 3,
  "file": "css/css-content/attr-case-sensitivity-001.html"
}
```

## style[0]

```css

div#gencon:before { content: attr(foo) attr(Foo) attr(fOO)
                             attr(bar) attr(Bar) attr(bAR)
                             attr(baz) attr(Baz) attr(BAZ) }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
