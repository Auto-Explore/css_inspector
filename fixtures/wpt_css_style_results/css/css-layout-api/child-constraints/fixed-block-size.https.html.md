# css/css-layout-api/child-constraints/fixed-block-size.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/fixed-block-size.https.html"
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

  --fixed-block-size: 10;

  --inline-size-expected: 3;
  --block-size-expected: 10;
}

.vrl {
  writing-mode: vertical-rl;
  visibility: hidden;
  width: 3px;
  height: 2px;

  --fixed-block-size: 10;

  --inline-size-expected: 3;
  --block-size-expected: 10;
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
