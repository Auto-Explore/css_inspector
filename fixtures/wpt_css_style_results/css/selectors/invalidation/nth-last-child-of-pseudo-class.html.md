# css/selectors/invalidation/nth-last-child-of-pseudo-class.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/nth-last-child-of-pseudo-class.html"
}
```

## style[0]

```css

  p:nth-last-child(odd of :defined) {
    color: green;
  }

  not-defined, my-element {
    display: block;
    margin-block: 1em;
    margin-inline: 0;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
