# css/css-layout-api/position-fragment/vrl-ltr.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/position-fragment/vrl-ltr.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  width: 100px;
  height: 100px;
}

.test {
  writing-mode: vertical-rl;
  direction: ltr;
}

.child-1 {
  background: rebeccapurple;
  width: 10px;
  height: 20px;

  --inline-offset: 25;
  --block-offset: 85;
}

.child-2 {
  writing-mode: vertical-rl;
  background: rebeccapurple;
  width: 15px;
  height: 25px;

  --inline-offset: 60;
  --block-offset: 35;
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
