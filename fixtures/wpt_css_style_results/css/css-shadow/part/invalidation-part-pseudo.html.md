# css/css-shadow/part/invalidation-part-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/invalidation-part-pseudo.html"
}
```

## style[0]

```css

/* NOTE: Even though it might be tempting to use :focus instead, because we
   can more easily add that state programmatically, that'd defeat the point
   of the test, since :focus / :focus-visible has default styles which
   invalidate the element's style anyways */
#host::part(a):hover {
  background: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    div {
      width: 100px;
      height: 100px;
      background: black;
      color: white;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
