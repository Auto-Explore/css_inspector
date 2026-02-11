# css/selectors/nth-child-of-compound-selector.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-of-compound-selector.html"
}
```

## style[0]

```css

/* 3n of paragraph with the class foobar but for which foobar is not the only class. */
p:nth-child(3n+1 of p.foobar:not([class=foobar])) {
    background-color: lime;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
