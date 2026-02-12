# css/selectors/is-where-pseudo-elements.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-where-pseudo-elements.html"
}
```

## style[0]

```css

  :is(#a, #c) + :is(main :is(#b, #d))::before {
    content: "before ";
    color: green;
  }
  :is(#d + div, #d ~ #h)::after {
    content: " after";
    color: green;
  }
  :is(main > #a, #b)::first-letter {
    color: blue;
  }
  :is(:where(main > div + #g, #k + #l))::first-line {
    color: magenta;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
