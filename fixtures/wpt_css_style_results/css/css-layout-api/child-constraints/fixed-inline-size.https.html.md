# css/css-layout-api/child-constraints/fixed-inline-size.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/fixed-inline-size.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  width: 100px;
}

.htb {
  writing-mode: horizontal-tb;
  visibility: hidden;
  width: 3px;
  height: 2px;

  --fixed-inline-size: 10;

  --inline-size-expected: 10;
  --block-size-expected: 2;
}

.vrl {
  writing-mode: vertical-rl;
  visibility: hidden;
  width: 3px;
  height: 2px;

  --fixed-inline-size: 10;

  --inline-size-expected: 10;
  --block-size-expected: 2;
}

@supports (display: layout(test)) {
  .test {
    background: green;
    display: layout(test);
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
