# css/selectors/not-default-ns-001.html

```json
{
  "format_version": 3,
  "file": "css/selectors/not-default-ns-001.html"
}
```

## style[0]

```css

@namespace url("http://www.w3.org/2000/svg");

*|div {
  width: 100px;
  height: 100px;
  background: red;
}

/* Type selector, so ns should apply and this should match */
*|*:not(div) {
  display: none;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
