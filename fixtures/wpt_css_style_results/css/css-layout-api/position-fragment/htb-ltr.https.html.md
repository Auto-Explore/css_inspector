# css/css-layout-api/position-fragment/htb-ltr.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/position-fragment/htb-ltr.https.html"
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
  writing-mode: horizontal-tb;
  direction: ltr;
}

.child-1 {
  background: rebeccapurple;
  width: 10px;
  height: 20px;

  --inline-offset: 5;
  --block-offset: 25;
}

.child-2 {
  writing-mode: vertical-rl;
  background: rebeccapurple;
  width: 15px;
  height: 25px;

  --inline-offset: 50;
  --block-offset: 60;
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
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
