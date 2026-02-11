# css/css-values/ex-calc-expression-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ex-calc-expression-001.html"
}
```

## style[0]

```css

/*
   This is a regression test for:
     https://github.com/servo/servo/pull/18807
     https://bugzilla.mozilla.org/show_bug.cgi?id=1407092
 */
div {
  width: calc(1ex + 1ex);
  height: calc(1ex + 1ex);
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
