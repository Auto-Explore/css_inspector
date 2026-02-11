# css/selectors/selectors-4/lang-023.html

```json
{
  "format_version": 3,
  "file": "css/selectors/selectors-4/lang-023.html"
}
```

## style[0]

```css

div.test { color: red; }
:lang("x") { color: green; } /* not a well-formed lang tag, but matches per
                                the Extended Filtering algorithm */
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
