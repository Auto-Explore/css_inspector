# css/css-flexbox/dynamic-isize-change-004.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/dynamic-isize-change-004.html"
}
```

## style[0]

```css

#flexbox {
  display: flex;
  inline-size: 100px;
  block-size: 100px;
  font: 50px/1 Ahem;
  color: green;
  word-break: break-all;
}

#flexbox > div {
  /* The following flex-basis and padding will give our flex item a border-box
     inline-size of 100px, both before and after this test's dynamic
     modification. */
  flex-basis: 50%;
  padding-right: calc(100px - 50%);
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
