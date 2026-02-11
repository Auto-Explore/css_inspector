# css/selectors/has-sibling-chrome-crash.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-sibling-chrome-crash.html"
}
```

## style[0]

```css

  :has(> :where(label:first-child + [a="a"]:only-of-type,
    [a="a"]:only-of-type + label:last-child)) label:last-child {
      margin-inline: 1em;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
