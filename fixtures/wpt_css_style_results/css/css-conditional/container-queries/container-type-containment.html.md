# css/css-conditional/container-queries/container-type-containment.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-type-containment.html"
}
```

## style[0]

```css

  /* Note: background colors have no impact on the test result. They are
           present to make it easier to visually verify that the test
           does the right thing. */
  .square {
    width: 50px;
    height: 50px;
    background: tomato;
  }
  .half {
    width: 25px;
    height: 50px;
    background: red;
  }
  div > div:nth-of-type(1) { background: skyblue; }
  div > div:nth-of-type(2) { background: hotpink; }
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

  #ref4::before, #child4::before {
    content: counter(foo);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
